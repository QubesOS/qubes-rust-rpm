// GENERATED by genarch.awk, do not edit

fn arch_to_archnum(arch: &[u8]) -> Option<u16> {
    match &arch.to_ascii_lowercase()[..] {
        b"noarch" => Some(255),
        b"athlon" => Some(1),
        b"geode" => Some(1),
        b"pentium4" => Some(1),
        b"pentium3" => Some(1),
        b"i686" => Some(1),
        b"i586" => Some(1),
        b"i486" => Some(1),
        b"i386" => Some(1),
        b"x86_64" => Some(1),
        b"amd64" => Some(1),
        b"ia32e" => Some(1),
        b"em64t" => Some(1),
        b"alpha" => Some(2),
        b"alphaev5" => Some(2),
        b"alphaev56" => Some(2),
        b"alphaev6" => Some(2),
        b"alphaev67" => Some(2),
        b"sun4u" => Some(2),
        b"sparc64" => Some(2),
        b"sparc64v" => Some(2),
        b"sparc" => Some(3),
        b"sun4" => Some(3),
        b"sun4m" => Some(3),
        b"sun4c" => Some(3),
        b"sun4d" => Some(3),
        b"sparcv8" => Some(3),
        b"sparcv9" => Some(3),
        b"sparcv9v" => Some(3),
        b"mips" => Some(4),
        b"mipsel" => Some(4),
        b"ppc" => Some(5),
        b"ppc8260" => Some(5),
        b"ppc8560" => Some(5),
        b"ppc32dy4" => Some(5),
        b"ppciseries" => Some(5),
        b"ppcpseries" => Some(5),
        b"m68k" => Some(6),
        b"ip" => Some(7),
        b"sgi" => Some(7),
        b"rs6000" => Some(8),
        b"ia64" => Some(9),
        b"mips64" => Some(11),
        b"mips64el" => Some(11),
        b"armv3l" => Some(12),
        b"armv4b" => Some(12),
        b"armv4l" => Some(12),
        b"armv5tl" => Some(12),
        b"armv5tel" => Some(12),
        b"armv5tejl" => Some(12),
        b"armv6l" => Some(12),
        b"armv6hl" => Some(12),
        b"armv7l" => Some(12),
        b"armv7hl" => Some(12),
        b"armv7hnl" => Some(12),
        b"armv8l" => Some(12),
        b"armv8hl" => Some(12),
        b"m68kmint" => Some(13),
        b"atarist" => Some(13),
        b"atariste" => Some(13),
        b"ataritt" => Some(13),
        b"falcon" => Some(13),
        b"atariclone" => Some(13),
        b"milan" => Some(13),
        b"hades" => Some(13),
        b"s390" => Some(14),
        b"i370" => Some(14),
        b"s390x" => Some(15),
        b"ppc64" => Some(16),
        b"ppc64le" => Some(16),
        b"ppc64pseries" => Some(16),
        b"ppc64iseries" => Some(16),
        b"ppc64p7" => Some(16),
        b"sh" => Some(17),
        b"sh3" => Some(17),
        b"sh4" => Some(17),
        b"sh4a" => Some(17),
        b"xtensa" => Some(18),
        b"aarch64" => Some(19),
        b"mipsr6" => Some(20),
        b"mipsr6el" => Some(20),
        b"mips64r6" => Some(21),
        b"mips64r6el" => Some(21),
        b"riscv" => Some(22),
        b"riscv64" => Some(22),
        _ => None,
    }
}

fn os_to_osnum(os: &[u8]) -> Option<u16> {
    match &os.to_ascii_lowercase()[..] {
        b"linux" => Some(1),
        b"irix" => Some(2),
        b"sunos5" => Some(3),
        b"solaris" => Some(3),
        b"sunos4" => Some(4),
        b"sunos" => Some(4),
        b"amigaos" => Some(5),
        b"aix" => Some(5),
        b"hp-ux" => Some(6),
        b"hpux10" => Some(6),
        b"osf1" => Some(7),
        b"freebsd" => Some(8),
        b"irix64" => Some(10),
        b"nextstep" => Some(11),
        b"bsd_os" => Some(12),
        b"bsdi" => Some(12),
        b"machten" => Some(13),
        b"cygwin32_nt" => Some(14),
        b"cygwin32" => Some(14),
        b"cygwin32_95" => Some(15),
        b"mint" => Some(17),
        b"freemint" => Some(17),
        b"darwin" => Some(21),
        b"macosx" => Some(21),
        _ => None,
    }
}
