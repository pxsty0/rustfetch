use regex::Regex;
use std::env;
use std::path::Path;
use std::process::Command;
use sysinfo::System;

#[derive(Default, Debug, Clone)]
pub struct Info {
    pub user: String,
    pub host: String,
    pub os_name: String,
    pub os_pretty: String,
    pub kernel: String,
    pub uptime: String,
    pub shell: String,
    pub terminal: String,
    pub packages: String,
    pub resolution: String,
    pub de_wm: String,
    pub cpu: String,
    pub gpu: String,
    pub memory: String,
}

pub fn gather_info() -> Info {
    let mut s = System::new_all();
    s.refresh_all();

    let user = whoami::username();
    let host = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string());

    let os = os_info::get();
    let os_name = os.os_type().to_string();
    let os_pretty = format!("{} {}", os.os_type(), os.version());

    let kernel = kernel_version();
    let uptime = format_uptime(System::uptime());
    let shell = detect_shell();
    let terminal = detect_terminal();
    let packages = count_packages();
    let resolution = detect_resolution();
    let de_wm = detect_de_wm();

    let cpu = if let Some(cpu) = s.cpus().first() {
        format!("{} ({:.0} MHz)", cpu.brand(), cpu.frequency())
    } else {
        "-".to_string()
    };
    let gpu = detect_gpu();

    let total = s.total_memory();
    let used = s.used_memory();
    let memory = format!(
        "{:.1} GiB / {:.1} GiB",
        bytes_to_gib(used),
        bytes_to_gib(total)
    );

    Info {
        user,
        host,
        os_name,
        os_pretty,
        kernel,
        uptime,
        shell,
        terminal,
        packages,
        resolution,
        de_wm,
        cpu,
        gpu,
        memory,
    }
}

fn bytes_to_gib(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}

fn format_uptime(total_seconds: u64) -> String {
    let days = total_seconds / 86_400;
    let hours = (total_seconds % 86_400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{} gün", days));
    }
    if hours > 0 {
        parts.push(format!("{} saat", hours));
    }
    if minutes > 0 {
        parts.push(format!("{} dk", minutes));
    }
    if parts.is_empty() {
        parts.push("<1 dk".to_string());
    }
    parts.join(", ")
}

fn kernel_version() -> String {
    if cfg!(target_os = "windows") {
        if let Ok(out) = Command::new("cmd").args(["/C", "ver"]).output() {
            if let Ok(s) = String::from_utf8(out.stdout) {
                return s.trim().to_string();
            }
        }
        return "-".to_string();
    }
    Command::new("uname")
        .arg("-r")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "-".to_string())
}

fn detect_shell() -> String {
    if let Ok(shell) = env::var("SHELL") {
        return basename(&shell);
    }
    if cfg!(target_os = "windows") {
        if env::var("PSModulePath").is_ok() {
            return "powershell".to_string();
        }
        return env::var("ComSpec")
            .map(|s| basename(&s))
            .unwrap_or_else(|_| "cmd".to_string());
    }
    if let Ok(ppid) = env::var("PPID") {
        if let Ok(out) = Command::new("ps")
            .args(["-p", &ppid, "-o", "comm="])
            .output()
        {
            if let Ok(s) = String::from_utf8(out.stdout) {
                let name = s.trim();
                if !name.is_empty() {
                    return basename(name);
                }
            }
        }
    }
    "-".to_string()
}

fn detect_terminal() -> String {
    for key in [
        "TERM_PROGRAM",
        "TERMINAL_EMULATOR",
        "WEZTERM_EXECUTABLE",
        "ITERM_SESSION_ID",
        "TERM",
        "WT_SESSION",
    ] {
        if let Ok(v) = env::var(key) {
            if !v.is_empty() {
                return v;
            }
        }
    }
    if cfg!(target_os = "windows") {
        if env::var("WT_SESSION").is_ok() {
            return "Windows Terminal".to_string();
        }
        return "ConsoleHost".to_string();
    }
    if let Ok(ppid) = env::var("PPID") {
        if let Ok(out) = Command::new("ps")
            .args(["-p", &ppid, "-o", "comm="])
            .output()
        {
            if let Ok(s) = String::from_utf8(out.stdout) {
                let name = s.trim();
                if !name.is_empty() {
                    return basename(name);
                }
            }
        }
    }
    "-".to_string()
}

fn basename(path: &str) -> String {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
        .to_string()
}

fn count_packages() -> String {
    if cfg!(target_os = "macos") {
        if which::which("brew").is_ok() {
            let mut total = 0usize;
            if let Ok(out) = Command::new("brew").args(["list", "--formula"]).output() {
                if out.status.success() {
                    total += String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .filter(|l| !l.trim().is_empty())
                        .count();
                }
            }
            if let Ok(out) = Command::new("brew").args(["list", "--cask"]).output() {
                if out.status.success() {
                    total += String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .filter(|l| !l.trim().is_empty())
                        .count();
                }
            }
            if total > 0 {
                return format!("brew ({})", total);
            }
        }
        if which::which("port").is_ok() {
            if let Ok(out) = Command::new("port").args(["installed"]).output() {
                if out.status.success() {
                    let count = String::from_utf8_lossy(&out.stdout)
                        .lines()
                        .filter(|l| l.contains("@"))
                        .count();
                    return format!("ports ({})", count);
                }
            }
        }
        return "-".to_string();
    }
    if cfg!(target_os = "linux") {
        if which::which("pacman").is_ok() {
            if let Ok(out) = Command::new("bash")
                .args(["-lc", "pacman -Qq 2>/dev/null | wc -l"])
                .output()
            {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    return format!("pacman ({})", s.trim());
                }
            }
        }
        if which::which("dpkg").is_ok() {
            if let Ok(out) = Command::new("bash")
                .args(["-lc", "dpkg -l 2>/dev/null | grep '^ii' | wc -l"])
                .output()
            {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    return format!("dpkg ({})", s.trim());
                }
            }
        }
        if which::which("rpm").is_ok() {
            if let Ok(out) = Command::new("bash")
                .args(["-lc", "rpm -qa 2>/dev/null | wc -l"])
                .output()
            {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    return format!("rpm ({})", s.trim());
                }
            }
        }
        return "-".to_string();
    }
    if cfg!(target_os = "windows") {
        if which::which("choco").is_ok() {
            if let Ok(out) = Command::new("cmd")
                .args(["/C", "choco list -lo | find /v /c \"\""])
                .output()
            {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    return format!("choco ({})", s.trim());
                }
            }
        }
        if which::which("winget").is_ok() {
            if let Ok(out) = Command::new("cmd")
                .args(["/C", "winget list | find /v /c \"\""])
                .output()
            {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    return format!("winget ({})", s.trim());
                }
            }
        }
        return "-".to_string();
    }
    "-".to_string()
}

fn detect_resolution() -> String {
    if cfg!(target_os = "macos") {
        if let Ok(out) = Command::new("system_profiler")
            .args(["SPDisplaysDataType"])
            .output()
        {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                let re = Regex::new(r"Resolution: (\d+) x (\d+)").unwrap();
                let mut res = Vec::new();
                for cap in re.captures_iter(&text) {
                    res.push(format!("{}x{}", &cap[1], &cap[2]));
                }
                if !res.is_empty() {
                    return res.join(", ");
                }
            }
        }
        return "-".to_string();
    }
    if cfg!(target_os = "linux") {
        if which::which("xrandr").is_ok() {
            if let Ok(out) = Command::new("bash")
                .args(["-lc", "xrandr | grep '*' | awk '{print $1}' | paste -sd, -"])
                .output()
            {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    let v = s.trim();
                    if !v.is_empty() {
                        return v.to_string();
                    }
                }
            }
        }
        return "-".to_string();
    }
    if cfg!(target_os = "windows") {
        if let Ok(out) = Command::new("powershell").args([
            "-NoProfile",
            "-Command",
            "(Get-CimInstance -Namespace root\\cimv2 -ClassName Win32_VideoController | Select-Object -First 1 CurrentHorizontalResolution,CurrentVerticalResolution) | ForEach-Object { \"$($_.CurrentHorizontalResolution)x$($_.CurrentVerticalResolution)\" }",
        ]).output() {
            if let Ok(s) = String::from_utf8(out.stdout) { return s.trim().to_string(); }
        }
        return "-".to_string();
    }
    "-".to_string()
}

fn detect_de_wm() -> String {
    if cfg!(target_os = "macos") {
        return "Aqua".to_string();
    }
    if cfg!(target_os = "windows") {
        return "Explorer".to_string();
    }
    if let Ok(v) = env::var("XDG_CURRENT_DESKTOP") {
        return v;
    }
    if let Ok(v) = env::var("DESKTOP_SESSION") {
        return v;
    }
    "-".to_string()
}

fn detect_gpu() -> String {
    if cfg!(target_os = "macos") {
        if let Ok(out) = Command::new("system_profiler")
            .args(["SPDisplaysDataType"])
            .output()
        {
            if out.status.success() {
                let text = String::from_utf8_lossy(&out.stdout);
                let re = Regex::new(r"Chipset Model: (.+)").unwrap();
                if let Some(cap) = re.captures(&text) {
                    return cap[1].trim().to_string();
                }
            }
        }
        return "-".to_string();
    }
    if cfg!(target_os = "linux") {
        if which::which("lspci").is_ok() {
            if let Ok(out) = Command::new("bash")
                .args([
                    "-lc",
                    "lspci | grep -i 'vga\\|3d\\|2d' | head -n1 | cut -d: -f3-",
                ])
                .output()
            {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    return s.trim().to_string();
                }
            }
        }
        return "-".to_string();
    }
    if cfg!(target_os = "windows") {
        if let Ok(out) = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "(Get-CimInstance Win32_VideoController | Select-Object -First 1 Name).Name",
            ])
            .output()
        {
            if let Ok(s) = String::from_utf8(out.stdout) {
                return s.trim().to_string();
            }
        }
        return "-".to_string();
    }
    "-".to_string()
}
