package OpenSSL::safe::installdata;

use strict;
use warnings;
use Exporter;
our @ISA = qw(Exporter);
our @EXPORT = qw(
    @PREFIX
    @libdir
    @BINDIR @BINDIR_REL_PREFIX
    @LIBDIR @LIBDIR_REL_PREFIX
    @INCLUDEDIR @INCLUDEDIR_REL_PREFIX
    @APPLINKDIR @APPLINKDIR_REL_PREFIX
    @ENGINESDIR @ENGINESDIR_REL_LIBDIR
    @MODULESDIR @MODULESDIR_REL_LIBDIR
    @PKGCONFIGDIR @PKGCONFIGDIR_REL_LIBDIR
    @CMAKECONFIGDIR @CMAKECONFIGDIR_REL_LIBDIR
    $COMMENT $VERSION @LDLIBS
);

our $COMMENT                    = 'This file should be used when building against this OpenSSL build, and should never be installed';
our @PREFIX                     = ( '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src' );
our @libdir                     = ( '' );
our @BINDIR                     = ( '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src/apps' );
our @BINDIR_REL_PREFIX          = ( 'apps' );
our @LIBDIR                     = ( '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src' );
our @LIBDIR_REL_PREFIX          = ( '' );
our @INCLUDEDIR                 = ( '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src/include', '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src/include' );
our @INCLUDEDIR_REL_PREFIX      = ( 'include', './include' );
our @APPLINKDIR                 = ( '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src/ms' );
our @APPLINKDIR_REL_PREFIX      = ( 'ms' );
our @ENGINESDIR                 = ( '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src/engines' );
our @ENGINESDIR_REL_LIBDIR      = ( 'engines' );
our @MODULESDIR                 = ( '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src/providers' );
our @MODULESDIR_REL_LIBDIR      = ( 'providers' );
our @PKGCONFIGDIR               = ( '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src' );
our @PKGCONFIGDIR_REL_LIBDIR    = ( '.' );
our @CMAKECONFIGDIR             = ( '/opt/subgrid/cargo-tmp/release/build/openssl-sys-f64e644bb015c6a3/out/openssl-build/build/src' );
our @CMAKECONFIGDIR_REL_LIBDIR  = ( '.' );
our $VERSION                    = '3.6.2';
our @LDLIBS                     =
    # Unix and Windows use space separation, VMS uses comma separation
    $^O eq 'VMS'
    ? split(/ *, */, '-ldl -pthread ')
    : split(/ +/, '-ldl -pthread ');

1;
