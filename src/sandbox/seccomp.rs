use crate::errors::*;
use syscallz::{self, Context, Syscall, Action};

pub fn activate_stage1() -> Result<()> {
    let mut ctx = Context::init()?;

    ctx.allow_syscall(Syscall::futex)?;
    ctx.allow_syscall(Syscall::read)?;
    ctx.allow_syscall(Syscall::write)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::open)?;
    ctx.allow_syscall(Syscall::close)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::stat)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::stat64)?;
    ctx.allow_syscall(Syscall::fstat)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::fstat64)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::lstat)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::lstat64)?;
    ctx.allow_syscall(Syscall::statx)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::poll)?;
    #[cfg(target_arch = "aarch64")]
    ctx.allow_syscall(Syscall::ppoll)?;
    ctx.allow_syscall(Syscall::lseek)?; 
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::_llseek)?; 
    #[cfg(not(target_arch = "arm"))]
    ctx.allow_syscall(Syscall::mmap)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::mmap2)?;
    ctx.allow_syscall(Syscall::mprotect)?;
    ctx.allow_syscall(Syscall::mremap)?;
    ctx.allow_syscall(Syscall::munmap)?;
    ctx.allow_syscall(Syscall::rt_sigprocmask)?;
    ctx.allow_syscall(Syscall::ioctl)?;
    ctx.allow_syscall(Syscall::readv)?;
    ctx.allow_syscall(Syscall::socket)?;
    ctx.allow_syscall(Syscall::connect)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::send)?;
    ctx.allow_syscall(Syscall::sendto)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::recv)?;
    ctx.allow_syscall(Syscall::recvfrom)?;
    ctx.allow_syscall(Syscall::sendmsg)?;
    ctx.allow_syscall(Syscall::recvmsg)?;
    ctx.allow_syscall(Syscall::bind)?;
    ctx.allow_syscall(Syscall::getsockname)?;
    ctx.allow_syscall(Syscall::setsockopt)?;
    ctx.allow_syscall(Syscall::getsockopt)?;
    ctx.allow_syscall(Syscall::clone)?;
    ctx.allow_syscall(Syscall::uname)?;
    ctx.allow_syscall(Syscall::fcntl)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::fcntl64)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::getdents)?;
    ctx.allow_syscall(Syscall::chdir)?; 
    ctx.allow_syscall(Syscall::getuid)?; 
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::getuid32)?; 
    ctx.allow_syscall(Syscall::getgid)?; 
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::getgid32)?;
    ctx.allow_syscall(Syscall::geteuid)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::geteuid32)?;
    ctx.allow_syscall(Syscall::getegid)?; 
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::getegid32)?; 
    ctx.allow_syscall(Syscall::setuid)?; 
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::setuid32)?; 
    ctx.allow_syscall(Syscall::setgid)?; 
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::setgid32)?; 
    ctx.allow_syscall(Syscall::getgroups)?; 
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::getgroups32)?; 
    ctx.allow_syscall(Syscall::setgroups)?; 
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::setgroups32)?; 
    ctx.allow_syscall(Syscall::getresuid)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::getresuid32)?;
    ctx.allow_syscall(Syscall::getresgid)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::getresgid32)?;
    ctx.allow_syscall(Syscall::sigaltstack)?;
    ctx.allow_syscall(Syscall::prctl)?; 
    ctx.allow_syscall(Syscall::chroot)?; 
    ctx.allow_syscall(Syscall::sched_getaffinity)?;
    ctx.allow_syscall(Syscall::sched_yield)?;
    ctx.allow_syscall(Syscall::getdents64)?;
    ctx.allow_syscall(Syscall::clock_getres)?;
    ctx.allow_syscall(Syscall::exit)?;
    ctx.allow_syscall(Syscall::exit_group)?;
    ctx.allow_syscall(Syscall::set_robust_list)?;
    ctx.allow_syscall(Syscall::openat)?;
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::newfstatat)?;
    ctx.allow_syscall(Syscall::seccomp)?; 
    ctx.allow_syscall(Syscall::getrandom)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::pipe)?; 
    ctx.allow_syscall(Syscall::wait4)?;
    ctx.allow_syscall(Syscall::clock_gettime)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::clock_gettime64)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::gettimeofday)?;
    ctx.allow_syscall(Syscall::brk)?;
    ctx.allow_syscall(Syscall::madvise)?;
    ctx.allow_syscall(Syscall::membarrier)?;
    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::access)?; 
    ctx.allow_syscall(Syscall::faccessat)?; 
    ctx.allow_syscall(Syscall::eventfd2)?;

    ctx.load()?;

    info!("stage 1/2 is active");

    Ok(())
}

pub fn activate_stage2() -> Result<()> {
    let mut ctx = Context::init()?;

    ctx.allow_syscall(Syscall::futex)?;
    ctx.allow_syscall(Syscall::read)?;
    ctx.allow_syscall(Syscall::write)?;
    ctx.allow_syscall(Syscall::close)?;

    #[cfg(not(target_arch = "aarch64"))]
    ctx.allow_syscall(Syscall::poll)?;
    #[cfg(target_arch = "aarch64")]
    ctx.allow_syscall(Syscall::ppoll)?;
    #[cfg(not(target_arch = "arm"))]
    ctx.allow_syscall(Syscall::mmap)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::mmap2)?;
    ctx.allow_syscall(Syscall::mprotect)?;
    ctx.allow_syscall(Syscall::mremap)?;
    ctx.allow_syscall(Syscall::munmap)?;
    ctx.allow_syscall(Syscall::rt_sigprocmask)?;

    ctx.allow_syscall(Syscall::readv)?;

    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::recv)?;

    ctx.allow_syscall(Syscall::getsockname)?;
    ctx.allow_syscall(Syscall::setsockopt)?;
    ctx.allow_syscall(Syscall::getsockopt)?;
    ctx.allow_syscall(Syscall::clone)?;

    ctx.allow_syscall(Syscall::sigaltstack)?;
    ctx.allow_syscall(Syscall::sched_getaffinity)?;
    ctx.allow_syscall(Syscall::sched_yield)?;

    ctx.allow_syscall(Syscall::exit)?;
    ctx.allow_syscall(Syscall::exit_group)?;
    ctx.allow_syscall(Syscall::set_robust_list)?;

    ctx.allow_syscall(Syscall::clock_gettime)?;
    #[cfg(target_arch = "arm")]
    ctx.allow_syscall(Syscall::clock_gettime64)?;
    ctx.allow_syscall(Syscall::brk)?;
    ctx.allow_syscall(Syscall::madvise)?;
    ctx.allow_syscall(Syscall::membarrier)?;

    ctx.set_action_for_syscall(Action::Errno(1), Syscall::openat)?;
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    ctx.set_action_for_syscall(Action::Errno(1), Syscall::open)?;

    ctx.load()?;

    info!("stage 2/2 is active");

    Ok(())
}