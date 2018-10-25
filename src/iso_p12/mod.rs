/*** IMPORTS ***/
#![allow(dead_code)]
mod assp;
mod bxml;
mod chnl;
mod cinf;
mod co64;
mod cprt;
mod cslg;
mod ctts;
mod dinf;
mod dmix;
mod dref;
mod edts;
mod elng;
mod elst;
mod feci;
mod fecr;
mod fiin;
mod fire;
mod fpar;
mod free;
mod frma;
mod ftyp;
mod gitn;
mod hdlr;
mod hmhd;
mod idat;
mod iinf;
mod iloc;
mod ipro;
mod iref;
mod kind;
mod leva;
mod ludt;
mod mdat;
mod mdhd;
mod mdia;
mod meco;
mod mehd;
mod mere;
mod meta;
mod mfhd;
mod mfra;
mod mfro;
mod minf;
mod moof;
mod moov;
mod mvex;
mod mvhd;
mod nmhd;
mod padb;
mod paen;
mod pdin;
mod pitm;
mod prft;
mod rinf;
mod saio;
mod saiz;
mod sbgp;
mod schi;
mod schm;
mod sdtp;
mod segr;
mod sgpd;
mod sidx;
mod sinf;
mod skip;
mod smhd;
mod srpp;
mod ssix;
mod stbl;
mod stco;
mod stdp;
mod strd;
mod stri;
mod strk;
mod stsc;
mod stsd;
mod stsg;
mod stsh;
mod stss;
mod stsz;
mod stts;
mod stvi;
mod styp;
mod stz2;
mod subs;
mod tfdt;
mod tfhd;
mod tfra;
mod tkhd;
mod traf;
mod trak;
mod tref;
mod trep;
mod trex;
mod trgr;
mod trun;
mod tsel;
mod udta;
mod vmhd;
mod xml;

pub use self::assp::Assp;
pub use self::bxml::Bxml;
pub use self::chnl::Chnl;
pub use self::cinf::Cinf;
pub use self::co64::Co64;
pub use self::cprt::Cprt;
pub use self::cslg::Cslg;
pub use self::ctts::Ctts;
pub use self::dinf::Dinf;
pub use self::dmix::Dmix;
pub use self::dref::Dref;
pub use self::edts::Edts;
pub use self::elng::Elng;
pub use self::elst::Elst;
pub use self::feci::Feci;
pub use self::fecr::Fecr;
pub use self::fiin::Fiin;
pub use self::fire::Fire;
pub use self::fpar::Fpar;
pub use self::free::Free;
pub use self::frma::Frma;
pub use self::ftyp::Ftyp;
pub use self::gitn::Gitn;
pub use self::hdlr::Hdlr;
pub use self::hmhd::Hmhd;
pub use self::idat::Idat;
pub use self::iinf::Iinf;
pub use self::iloc::Iloc;
pub use self::ipro::Ipro;
pub use self::iref::Iref;
pub use self::kind::Kind;
pub use self::leva::Leva;
pub use self::ludt::Ludt;
pub use self::mdat::Mdat;
pub use self::mdhd::Mdhd;
pub use self::mdia::Mdia;
pub use self::meco::Meco;
pub use self::mehd::Mehd;
pub use self::mere::Mere;
pub use self::meta::Meta;
pub use self::mfhd::Mfhd;
pub use self::mfra::Mfra;
pub use self::mfro::Mfro;
pub use self::minf::Minf;
pub use self::moof::Moof;
pub use self::moov::Moov;
pub use self::mvex::Mvex;
pub use self::mvhd::Mvhd;
pub use self::nmhd::Nmhd;
pub use self::padb::Padb;
pub use self::paen::Paen;
pub use self::pdin::Pdin;
pub use self::pitm::Pitm;
pub use self::prft::Prft;
pub use self::rinf::Rinf;
pub use self::saio::Saio;
pub use self::saiz::Saiz;
pub use self::sbgp::Sbgp;
pub use self::schi::Schi;
pub use self::schm::Schm;
pub use self::sdtp::Sdtp;
pub use self::segr::Segr;
pub use self::sgpd::Sgpd;
pub use self::sidx::Sidx;
pub use self::sinf::Sinf;
pub use self::skip::Skip;
pub use self::smhd::Smhd;
pub use self::srpp::Srpp;
pub use self::ssix::Ssix;
pub use self::stbl::Stbl;
pub use self::stco::Stco;
pub use self::stdp::Stdp;
pub use self::strd::Strd;
pub use self::stri::Stri;
pub use self::strk::Strk;
pub use self::stsc::Stsc;
pub use self::stsd::Stsd;
pub use self::stsg::Stsg;
pub use self::stsh::Stsh;
pub use self::stss::Stss;
pub use self::stsz::Stsz;
pub use self::stts::Stts;
pub use self::stvi::Stvi;
pub use self::styp::Styp;
pub use self::stz2::Stz2;
pub use self::subs::Subs;
pub use self::tfdt::Tfdt;
pub use self::tfhd::Tfhd;
pub use self::tfra::Tfra;
pub use self::tkhd::Tkhd;
pub use self::traf::Traf;
pub use self::trak::Trak;
pub use self::tref::Tref;
pub use self::trep::Trep;
pub use self::trex::Trex;
pub use self::trgr::Trgr;
pub use self::trun::Trun;
pub use self::tsel::Tsel;
pub use self::udta::Udta;
pub use self::vmhd::Vmhd;
pub use self::xml::Xml;

/*** STATIC TYPES ***/
pub static ATOM_TYPES_NCHILDREN: &'static [&str] = &[
    "feci", "free", "frma", "ftyp", "idat", "mdat", "segr", "skip", "styp", "tref",
];

pub static ATOM_TYPES_WCHILDREN: &'static [&str] = &[
    "cinf", "dinf", "edts", "mdia", "meco", "mfra", "minf", "moof", "moov", "mvex", "paen", "rinf",
    "schi", "sinf", "stbl", "strd", "strk", "traf", "trak", "udta",
];

pub static FULL_ATOM_TYPES_NCHILDREN: &'static [&str] = &[
    "assp", "bxml", "chnl", "co64", "cprt", "cslg", "ctts", "dmix", "elng", "elst", "fecr", "fire",
    "fpar", "gitn", "hdlr", "hmhd", "iloc", "kind", "leva", "ludt", "mdhd", "mehd", "mere", "mfhd",
    "mfro", "mvhd", "nmhd", "padb", "pdin", "pitm", "prft", "saio", "saiz", "sbgp", "schm", "sdtp",
    "sidx", "smhd", "stco", "stdp", "stri", "stsc", "stsg", "stsh", "stss", "stsz", "stts", "stz2",
    "subs", "tfdt", "tfhd", "tfra", "tkhd", "trex", "trun", "tsel", "vmhd", "xml ",
];

pub static FULL_ATOM_TYPES_WCHILDREN: &'static [&str] = &[
    "dref", "fiin", "iinf", "ipro", "iref", "meta", "sgpd", "srpp", "ssix", "stsd", "stvi", "trep",
    "trgr",
];
