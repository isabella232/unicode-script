// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// NOTE: The following code was generated by "scripts/unicode.py", do not edit directly

#![allow(missing_docs, non_upper_case_globals, non_snake_case)]

/// The version of [Unicode](http://www.unicode.org/)
/// that this version of unicode-width is based on.
pub const UNICODE_VERSION: (u64, u64, u64) = (8, 0, 0);

pub use self::Script::*;

    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// Represents the Unicode character property **Script**.
    ///
    /// http://www.unicode.org/reports/tr24/
    pub enum Script {
        Unknown,
        Ahom,
        Anatolian_Hieroglyphs,
        Arabic,
        Armenian,
        Avestan,
        Balinese,
        Bamum,
        Bassa_Vah,
        Batak,
        Bengali,
        Bopomofo,
        Brahmi,
        Braille,
        Buginese,
        Buhid,
        Canadian_Aboriginal,
        Carian,
        Caucasian_Albanian,
        Chakma,
        Cham,
        Cherokee,
        Common,
        Coptic,
        Cuneiform,
        Cypriot,
        Cyrillic,
        Deseret,
        Devanagari,
        Duployan,
        Egyptian_Hieroglyphs,
        Elbasan,
        Ethiopic,
        Georgian,
        Glagolitic,
        Gothic,
        Grantha,
        Greek,
        Gujarati,
        Gurmukhi,
        Han,
        Hangul,
        Hanunoo,
        Hatran,
        Hebrew,
        Hiragana,
        Imperial_Aramaic,
        Inherited,
        Inscriptional_Pahlavi,
        Inscriptional_Parthian,
        Javanese,
        Kaithi,
        Kannada,
        Katakana,
        Kayah_Li,
        Kharoshthi,
        Khmer,
        Khojki,
        Khudawadi,
        Lao,
        Latin,
        Lepcha,
        Limbu,
        Linear_A,
        Linear_B,
        Lisu,
        Lycian,
        Lydian,
        Mahajani,
        Malayalam,
        Mandaic,
        Manichaean,
        Meetei_Mayek,
        Mende_Kikakui,
        Meroitic_Cursive,
        Meroitic_Hieroglyphs,
        Miao,
        Modi,
        Mongolian,
        Mro,
        Multani,
        Myanmar,
        Nabataean,
        New_Tai_Lue,
        Nko,
        Ogham,
        Ol_Chiki,
        Old_Hungarian,
        Old_Italic,
        Old_North_Arabian,
        Old_Permic,
        Old_Persian,
        Old_South_Arabian,
        Old_Turkic,
        Oriya,
        Osmanya,
        Pahawh_Hmong,
        Palmyrene,
        Pau_Cin_Hau,
        Phags_Pa,
        Phoenician,
        Psalter_Pahlavi,
        Rejang,
        Runic,
        Samaritan,
        Saurashtra,
        Sharada,
        Shavian,
        Siddham,
        SignWriting,
        Sinhala,
        Sora_Sompeng,
        Sundanese,
        Syloti_Nagri,
        Syriac,
        Tagalog,
        Tagbanwa,
        Tai_Le,
        Tai_Tham,
        Tai_Viet,
        Takri,
        Tamil,
        Telugu,
        Thaana,
        Thai,
        Tibetan,
        Tifinagh,
        Tirhuta,
        Ugaritic,
        Vai,
        Warang_Citi,
        Yi,
    }

    fn bsearch_range_value_table(c: char, r: &'static [(char, char, Script)]) -> Script {
        use ::std::cmp::Ordering::{Equal, Less, Greater};
        match r.binary_search_by(|&(lo, hi, _)| {
            if lo <= c && c <= hi { Equal }
            else if hi < c { Less }
            else { Greater }
        }) {
            Ok(idx) => {
                let (_, _, cat) = r[idx];
                cat
            }
            Err(_) => Unknown
        }
    }

    /// Find the script of a single char.
    pub fn get_script(c: char) -> Script {
        bsearch_range_value_table(c, script_table)
    }

    const script_table: &'static [(char, char, Script)] = &[
        ('\u{0}', '\u{40}', Common), ('\u{41}', '\u{5a}', Latin), ('\u{5b}', '\u{60}', Common),
        ('\u{61}', '\u{7a}', Latin), ('\u{7b}', '\u{a9}', Common), ('\u{aa}', '\u{aa}', Latin),
        ('\u{ab}', '\u{b9}', Common), ('\u{ba}', '\u{ba}', Latin), ('\u{bb}', '\u{bf}', Common),
        ('\u{c0}', '\u{d6}', Latin), ('\u{d7}', '\u{d7}', Common), ('\u{d8}', '\u{f6}', Latin),
        ('\u{f7}', '\u{f7}', Common), ('\u{f8}', '\u{2b8}', Latin), ('\u{2b9}', '\u{2df}', Common),
        ('\u{2e0}', '\u{2e4}', Latin), ('\u{2e5}', '\u{2e9}', Common), ('\u{2ea}', '\u{2eb}',
        Bopomofo), ('\u{2ec}', '\u{2ff}', Common), ('\u{300}', '\u{36f}', Inherited), ('\u{370}',
        '\u{373}', Greek), ('\u{374}', '\u{374}', Common), ('\u{375}', '\u{377}', Greek),
        ('\u{37a}', '\u{37d}', Greek), ('\u{37e}', '\u{37e}', Common), ('\u{37f}', '\u{37f}',
        Greek), ('\u{384}', '\u{384}', Greek), ('\u{385}', '\u{385}', Common), ('\u{386}',
        '\u{386}', Greek), ('\u{387}', '\u{387}', Common), ('\u{388}', '\u{38a}', Greek),
        ('\u{38c}', '\u{38c}', Greek), ('\u{38e}', '\u{3a1}', Greek), ('\u{3a3}', '\u{3e1}', Greek),
        ('\u{3e2}', '\u{3ef}', Coptic), ('\u{3f0}', '\u{3ff}', Greek), ('\u{400}', '\u{484}',
        Cyrillic), ('\u{485}', '\u{486}', Inherited), ('\u{487}', '\u{52f}', Cyrillic), ('\u{531}',
        '\u{556}', Armenian), ('\u{559}', '\u{55f}', Armenian), ('\u{561}', '\u{587}', Armenian),
        ('\u{589}', '\u{589}', Common), ('\u{58a}', '\u{58a}', Armenian), ('\u{58d}', '\u{58f}',
        Armenian), ('\u{591}', '\u{5c7}', Hebrew), ('\u{5d0}', '\u{5ea}', Hebrew), ('\u{5f0}',
        '\u{5f4}', Hebrew), ('\u{600}', '\u{604}', Arabic), ('\u{605}', '\u{605}', Common),
        ('\u{606}', '\u{60b}', Arabic), ('\u{60c}', '\u{60c}', Common), ('\u{60d}', '\u{61a}',
        Arabic), ('\u{61b}', '\u{61c}', Common), ('\u{61e}', '\u{61e}', Arabic), ('\u{61f}',
        '\u{61f}', Common), ('\u{620}', '\u{63f}', Arabic), ('\u{640}', '\u{640}', Common),
        ('\u{641}', '\u{64a}', Arabic), ('\u{64b}', '\u{655}', Inherited), ('\u{656}', '\u{66f}',
        Arabic), ('\u{670}', '\u{670}', Inherited), ('\u{671}', '\u{6dc}', Arabic), ('\u{6dd}',
        '\u{6dd}', Common), ('\u{6de}', '\u{6ff}', Arabic), ('\u{700}', '\u{70d}', Syriac),
        ('\u{70f}', '\u{74a}', Syriac), ('\u{74d}', '\u{74f}', Syriac), ('\u{750}', '\u{77f}',
        Arabic), ('\u{780}', '\u{7b1}', Thaana), ('\u{7c0}', '\u{7fa}', Nko), ('\u{800}', '\u{82d}',
        Samaritan), ('\u{830}', '\u{83e}', Samaritan), ('\u{840}', '\u{85b}', Mandaic), ('\u{85e}',
        '\u{85e}', Mandaic), ('\u{8a0}', '\u{8b4}', Arabic), ('\u{8e3}', '\u{8ff}', Arabic),
        ('\u{900}', '\u{950}', Devanagari), ('\u{951}', '\u{952}', Inherited), ('\u{953}',
        '\u{963}', Devanagari), ('\u{964}', '\u{965}', Common), ('\u{966}', '\u{97f}', Devanagari),
        ('\u{980}', '\u{983}', Bengali), ('\u{985}', '\u{98c}', Bengali), ('\u{98f}', '\u{990}',
        Bengali), ('\u{993}', '\u{9a8}', Bengali), ('\u{9aa}', '\u{9b0}', Bengali), ('\u{9b2}',
        '\u{9b2}', Bengali), ('\u{9b6}', '\u{9b9}', Bengali), ('\u{9bc}', '\u{9c4}', Bengali),
        ('\u{9c7}', '\u{9c8}', Bengali), ('\u{9cb}', '\u{9ce}', Bengali), ('\u{9d7}', '\u{9d7}',
        Bengali), ('\u{9dc}', '\u{9dd}', Bengali), ('\u{9df}', '\u{9e3}', Bengali), ('\u{9e6}',
        '\u{9fb}', Bengali), ('\u{a01}', '\u{a03}', Gurmukhi), ('\u{a05}', '\u{a0a}', Gurmukhi),
        ('\u{a0f}', '\u{a10}', Gurmukhi), ('\u{a13}', '\u{a28}', Gurmukhi), ('\u{a2a}', '\u{a30}',
        Gurmukhi), ('\u{a32}', '\u{a33}', Gurmukhi), ('\u{a35}', '\u{a36}', Gurmukhi), ('\u{a38}',
        '\u{a39}', Gurmukhi), ('\u{a3c}', '\u{a3c}', Gurmukhi), ('\u{a3e}', '\u{a42}', Gurmukhi),
        ('\u{a47}', '\u{a48}', Gurmukhi), ('\u{a4b}', '\u{a4d}', Gurmukhi), ('\u{a51}', '\u{a51}',
        Gurmukhi), ('\u{a59}', '\u{a5c}', Gurmukhi), ('\u{a5e}', '\u{a5e}', Gurmukhi), ('\u{a66}',
        '\u{a75}', Gurmukhi), ('\u{a81}', '\u{a83}', Gujarati), ('\u{a85}', '\u{a8d}', Gujarati),
        ('\u{a8f}', '\u{a91}', Gujarati), ('\u{a93}', '\u{aa8}', Gujarati), ('\u{aaa}', '\u{ab0}',
        Gujarati), ('\u{ab2}', '\u{ab3}', Gujarati), ('\u{ab5}', '\u{ab9}', Gujarati), ('\u{abc}',
        '\u{ac5}', Gujarati), ('\u{ac7}', '\u{ac9}', Gujarati), ('\u{acb}', '\u{acd}', Gujarati),
        ('\u{ad0}', '\u{ad0}', Gujarati), ('\u{ae0}', '\u{ae3}', Gujarati), ('\u{ae6}', '\u{af1}',
        Gujarati), ('\u{af9}', '\u{af9}', Gujarati), ('\u{b01}', '\u{b03}', Oriya), ('\u{b05}',
        '\u{b0c}', Oriya), ('\u{b0f}', '\u{b10}', Oriya), ('\u{b13}', '\u{b28}', Oriya), ('\u{b2a}',
        '\u{b30}', Oriya), ('\u{b32}', '\u{b33}', Oriya), ('\u{b35}', '\u{b39}', Oriya), ('\u{b3c}',
        '\u{b44}', Oriya), ('\u{b47}', '\u{b48}', Oriya), ('\u{b4b}', '\u{b4d}', Oriya), ('\u{b56}',
        '\u{b57}', Oriya), ('\u{b5c}', '\u{b5d}', Oriya), ('\u{b5f}', '\u{b63}', Oriya), ('\u{b66}',
        '\u{b77}', Oriya), ('\u{b82}', '\u{b83}', Tamil), ('\u{b85}', '\u{b8a}', Tamil), ('\u{b8e}',
        '\u{b90}', Tamil), ('\u{b92}', '\u{b95}', Tamil), ('\u{b99}', '\u{b9a}', Tamil), ('\u{b9c}',
        '\u{b9c}', Tamil), ('\u{b9e}', '\u{b9f}', Tamil), ('\u{ba3}', '\u{ba4}', Tamil), ('\u{ba8}',
        '\u{baa}', Tamil), ('\u{bae}', '\u{bb9}', Tamil), ('\u{bbe}', '\u{bc2}', Tamil), ('\u{bc6}',
        '\u{bc8}', Tamil), ('\u{bca}', '\u{bcd}', Tamil), ('\u{bd0}', '\u{bd0}', Tamil), ('\u{bd7}',
        '\u{bd7}', Tamil), ('\u{be6}', '\u{bfa}', Tamil), ('\u{c00}', '\u{c03}', Telugu),
        ('\u{c05}', '\u{c0c}', Telugu), ('\u{c0e}', '\u{c10}', Telugu), ('\u{c12}', '\u{c28}',
        Telugu), ('\u{c2a}', '\u{c39}', Telugu), ('\u{c3d}', '\u{c44}', Telugu), ('\u{c46}',
        '\u{c48}', Telugu), ('\u{c4a}', '\u{c4d}', Telugu), ('\u{c55}', '\u{c56}', Telugu),
        ('\u{c58}', '\u{c5a}', Telugu), ('\u{c60}', '\u{c63}', Telugu), ('\u{c66}', '\u{c6f}',
        Telugu), ('\u{c78}', '\u{c7f}', Telugu), ('\u{c81}', '\u{c83}', Kannada), ('\u{c85}',
        '\u{c8c}', Kannada), ('\u{c8e}', '\u{c90}', Kannada), ('\u{c92}', '\u{ca8}', Kannada),
        ('\u{caa}', '\u{cb3}', Kannada), ('\u{cb5}', '\u{cb9}', Kannada), ('\u{cbc}', '\u{cc4}',
        Kannada), ('\u{cc6}', '\u{cc8}', Kannada), ('\u{cca}', '\u{ccd}', Kannada), ('\u{cd5}',
        '\u{cd6}', Kannada), ('\u{cde}', '\u{cde}', Kannada), ('\u{ce0}', '\u{ce3}', Kannada),
        ('\u{ce6}', '\u{cef}', Kannada), ('\u{cf1}', '\u{cf2}', Kannada), ('\u{d01}', '\u{d03}',
        Malayalam), ('\u{d05}', '\u{d0c}', Malayalam), ('\u{d0e}', '\u{d10}', Malayalam),
        ('\u{d12}', '\u{d3a}', Malayalam), ('\u{d3d}', '\u{d44}', Malayalam), ('\u{d46}', '\u{d48}',
        Malayalam), ('\u{d4a}', '\u{d4e}', Malayalam), ('\u{d57}', '\u{d57}', Malayalam),
        ('\u{d5f}', '\u{d63}', Malayalam), ('\u{d66}', '\u{d75}', Malayalam), ('\u{d79}', '\u{d7f}',
        Malayalam), ('\u{d82}', '\u{d83}', Sinhala), ('\u{d85}', '\u{d96}', Sinhala), ('\u{d9a}',
        '\u{db1}', Sinhala), ('\u{db3}', '\u{dbb}', Sinhala), ('\u{dbd}', '\u{dbd}', Sinhala),
        ('\u{dc0}', '\u{dc6}', Sinhala), ('\u{dca}', '\u{dca}', Sinhala), ('\u{dcf}', '\u{dd4}',
        Sinhala), ('\u{dd6}', '\u{dd6}', Sinhala), ('\u{dd8}', '\u{ddf}', Sinhala), ('\u{de6}',
        '\u{def}', Sinhala), ('\u{df2}', '\u{df4}', Sinhala), ('\u{e01}', '\u{e3a}', Thai),
        ('\u{e3f}', '\u{e3f}', Common), ('\u{e40}', '\u{e5b}', Thai), ('\u{e81}', '\u{e82}', Lao),
        ('\u{e84}', '\u{e84}', Lao), ('\u{e87}', '\u{e88}', Lao), ('\u{e8a}', '\u{e8a}', Lao),
        ('\u{e8d}', '\u{e8d}', Lao), ('\u{e94}', '\u{e97}', Lao), ('\u{e99}', '\u{e9f}', Lao),
        ('\u{ea1}', '\u{ea3}', Lao), ('\u{ea5}', '\u{ea5}', Lao), ('\u{ea7}', '\u{ea7}', Lao),
        ('\u{eaa}', '\u{eab}', Lao), ('\u{ead}', '\u{eb9}', Lao), ('\u{ebb}', '\u{ebd}', Lao),
        ('\u{ec0}', '\u{ec4}', Lao), ('\u{ec6}', '\u{ec6}', Lao), ('\u{ec8}', '\u{ecd}', Lao),
        ('\u{ed0}', '\u{ed9}', Lao), ('\u{edc}', '\u{edf}', Lao), ('\u{f00}', '\u{f47}', Tibetan),
        ('\u{f49}', '\u{f6c}', Tibetan), ('\u{f71}', '\u{f97}', Tibetan), ('\u{f99}', '\u{fbc}',
        Tibetan), ('\u{fbe}', '\u{fcc}', Tibetan), ('\u{fce}', '\u{fd4}', Tibetan), ('\u{fd5}',
        '\u{fd8}', Common), ('\u{fd9}', '\u{fda}', Tibetan), ('\u{1000}', '\u{109f}', Myanmar),
        ('\u{10a0}', '\u{10c5}', Georgian), ('\u{10c7}', '\u{10c7}', Georgian), ('\u{10cd}',
        '\u{10cd}', Georgian), ('\u{10d0}', '\u{10fa}', Georgian), ('\u{10fb}', '\u{10fb}', Common),
        ('\u{10fc}', '\u{10ff}', Georgian), ('\u{1100}', '\u{11ff}', Hangul), ('\u{1200}',
        '\u{1248}', Ethiopic), ('\u{124a}', '\u{124d}', Ethiopic), ('\u{1250}', '\u{1256}',
        Ethiopic), ('\u{1258}', '\u{1258}', Ethiopic), ('\u{125a}', '\u{125d}', Ethiopic),
        ('\u{1260}', '\u{1288}', Ethiopic), ('\u{128a}', '\u{128d}', Ethiopic), ('\u{1290}',
        '\u{12b0}', Ethiopic), ('\u{12b2}', '\u{12b5}', Ethiopic), ('\u{12b8}', '\u{12be}',
        Ethiopic), ('\u{12c0}', '\u{12c0}', Ethiopic), ('\u{12c2}', '\u{12c5}', Ethiopic),
        ('\u{12c8}', '\u{12d6}', Ethiopic), ('\u{12d8}', '\u{1310}', Ethiopic), ('\u{1312}',
        '\u{1315}', Ethiopic), ('\u{1318}', '\u{135a}', Ethiopic), ('\u{135d}', '\u{137c}',
        Ethiopic), ('\u{1380}', '\u{1399}', Ethiopic), ('\u{13a0}', '\u{13f5}', Cherokee),
        ('\u{13f8}', '\u{13fd}', Cherokee), ('\u{1400}', '\u{167f}', Canadian_Aboriginal),
        ('\u{1680}', '\u{169c}', Ogham), ('\u{16a0}', '\u{16ea}', Runic), ('\u{16eb}', '\u{16ed}',
        Common), ('\u{16ee}', '\u{16f8}', Runic), ('\u{1700}', '\u{170c}', Tagalog), ('\u{170e}',
        '\u{1714}', Tagalog), ('\u{1720}', '\u{1734}', Hanunoo), ('\u{1735}', '\u{1736}', Common),
        ('\u{1740}', '\u{1753}', Buhid), ('\u{1760}', '\u{176c}', Tagbanwa), ('\u{176e}',
        '\u{1770}', Tagbanwa), ('\u{1772}', '\u{1773}', Tagbanwa), ('\u{1780}', '\u{17dd}', Khmer),
        ('\u{17e0}', '\u{17e9}', Khmer), ('\u{17f0}', '\u{17f9}', Khmer), ('\u{1800}', '\u{1801}',
        Mongolian), ('\u{1802}', '\u{1803}', Common), ('\u{1804}', '\u{1804}', Mongolian),
        ('\u{1805}', '\u{1805}', Common), ('\u{1806}', '\u{180e}', Mongolian), ('\u{1810}',
        '\u{1819}', Mongolian), ('\u{1820}', '\u{1877}', Mongolian), ('\u{1880}', '\u{18aa}',
        Mongolian), ('\u{18b0}', '\u{18f5}', Canadian_Aboriginal), ('\u{1900}', '\u{191e}', Limbu),
        ('\u{1920}', '\u{192b}', Limbu), ('\u{1930}', '\u{193b}', Limbu), ('\u{1940}', '\u{1940}',
        Limbu), ('\u{1944}', '\u{194f}', Limbu), ('\u{1950}', '\u{196d}', Tai_Le), ('\u{1970}',
        '\u{1974}', Tai_Le), ('\u{1980}', '\u{19ab}', New_Tai_Lue), ('\u{19b0}', '\u{19c9}',
        New_Tai_Lue), ('\u{19d0}', '\u{19da}', New_Tai_Lue), ('\u{19de}', '\u{19df}', New_Tai_Lue),
        ('\u{19e0}', '\u{19ff}', Khmer), ('\u{1a00}', '\u{1a1b}', Buginese), ('\u{1a1e}',
        '\u{1a1f}', Buginese), ('\u{1a20}', '\u{1a5e}', Tai_Tham), ('\u{1a60}', '\u{1a7c}',
        Tai_Tham), ('\u{1a7f}', '\u{1a89}', Tai_Tham), ('\u{1a90}', '\u{1a99}', Tai_Tham),
        ('\u{1aa0}', '\u{1aad}', Tai_Tham), ('\u{1ab0}', '\u{1abe}', Inherited), ('\u{1b00}',
        '\u{1b4b}', Balinese), ('\u{1b50}', '\u{1b7c}', Balinese), ('\u{1b80}', '\u{1bbf}',
        Sundanese), ('\u{1bc0}', '\u{1bf3}', Batak), ('\u{1bfc}', '\u{1bff}', Batak), ('\u{1c00}',
        '\u{1c37}', Lepcha), ('\u{1c3b}', '\u{1c49}', Lepcha), ('\u{1c4d}', '\u{1c4f}', Lepcha),
        ('\u{1c50}', '\u{1c7f}', Ol_Chiki), ('\u{1cc0}', '\u{1cc7}', Sundanese), ('\u{1cd0}',
        '\u{1cd2}', Inherited), ('\u{1cd3}', '\u{1cd3}', Common), ('\u{1cd4}', '\u{1ce0}',
        Inherited), ('\u{1ce1}', '\u{1ce1}', Common), ('\u{1ce2}', '\u{1ce8}', Inherited),
        ('\u{1ce9}', '\u{1cec}', Common), ('\u{1ced}', '\u{1ced}', Inherited), ('\u{1cee}',
        '\u{1cf3}', Common), ('\u{1cf4}', '\u{1cf4}', Inherited), ('\u{1cf5}', '\u{1cf6}', Common),
        ('\u{1cf8}', '\u{1cf9}', Inherited), ('\u{1d00}', '\u{1d25}', Latin), ('\u{1d26}',
        '\u{1d2a}', Greek), ('\u{1d2b}', '\u{1d2b}', Cyrillic), ('\u{1d2c}', '\u{1d5c}', Latin),
        ('\u{1d5d}', '\u{1d61}', Greek), ('\u{1d62}', '\u{1d65}', Latin), ('\u{1d66}', '\u{1d6a}',
        Greek), ('\u{1d6b}', '\u{1d77}', Latin), ('\u{1d78}', '\u{1d78}', Cyrillic), ('\u{1d79}',
        '\u{1dbe}', Latin), ('\u{1dbf}', '\u{1dbf}', Greek), ('\u{1dc0}', '\u{1df5}', Inherited),
        ('\u{1dfc}', '\u{1dff}', Inherited), ('\u{1e00}', '\u{1eff}', Latin), ('\u{1f00}',
        '\u{1f15}', Greek), ('\u{1f18}', '\u{1f1d}', Greek), ('\u{1f20}', '\u{1f45}', Greek),
        ('\u{1f48}', '\u{1f4d}', Greek), ('\u{1f50}', '\u{1f57}', Greek), ('\u{1f59}', '\u{1f59}',
        Greek), ('\u{1f5b}', '\u{1f5b}', Greek), ('\u{1f5d}', '\u{1f5d}', Greek), ('\u{1f5f}',
        '\u{1f7d}', Greek), ('\u{1f80}', '\u{1fb4}', Greek), ('\u{1fb6}', '\u{1fc4}', Greek),
        ('\u{1fc6}', '\u{1fd3}', Greek), ('\u{1fd6}', '\u{1fdb}', Greek), ('\u{1fdd}', '\u{1fef}',
        Greek), ('\u{1ff2}', '\u{1ff4}', Greek), ('\u{1ff6}', '\u{1ffe}', Greek), ('\u{2000}',
        '\u{200b}', Common), ('\u{200c}', '\u{200d}', Inherited), ('\u{200e}', '\u{2064}', Common),
        ('\u{2066}', '\u{2070}', Common), ('\u{2071}', '\u{2071}', Latin), ('\u{2074}', '\u{207e}',
        Common), ('\u{207f}', '\u{207f}', Latin), ('\u{2080}', '\u{208e}', Common), ('\u{2090}',
        '\u{209c}', Latin), ('\u{20a0}', '\u{20be}', Common), ('\u{20d0}', '\u{20f0}', Inherited),
        ('\u{2100}', '\u{2125}', Common), ('\u{2126}', '\u{2126}', Greek), ('\u{2127}', '\u{2129}',
        Common), ('\u{212a}', '\u{212b}', Latin), ('\u{212c}', '\u{2131}', Common), ('\u{2132}',
        '\u{2132}', Latin), ('\u{2133}', '\u{214d}', Common), ('\u{214e}', '\u{214e}', Latin),
        ('\u{214f}', '\u{215f}', Common), ('\u{2160}', '\u{2188}', Latin), ('\u{2189}', '\u{218b}',
        Common), ('\u{2190}', '\u{23fa}', Common), ('\u{2400}', '\u{2426}', Common), ('\u{2440}',
        '\u{244a}', Common), ('\u{2460}', '\u{27ff}', Common), ('\u{2800}', '\u{28ff}', Braille),
        ('\u{2900}', '\u{2b73}', Common), ('\u{2b76}', '\u{2b95}', Common), ('\u{2b98}', '\u{2bb9}',
        Common), ('\u{2bbd}', '\u{2bc8}', Common), ('\u{2bca}', '\u{2bd1}', Common), ('\u{2bec}',
        '\u{2bef}', Common), ('\u{2c00}', '\u{2c2e}', Glagolitic), ('\u{2c30}', '\u{2c5e}',
        Glagolitic), ('\u{2c60}', '\u{2c7f}', Latin), ('\u{2c80}', '\u{2cf3}', Coptic), ('\u{2cf9}',
        '\u{2cff}', Coptic), ('\u{2d00}', '\u{2d25}', Georgian), ('\u{2d27}', '\u{2d27}', Georgian),
        ('\u{2d2d}', '\u{2d2d}', Georgian), ('\u{2d30}', '\u{2d67}', Tifinagh), ('\u{2d6f}',
        '\u{2d70}', Tifinagh), ('\u{2d7f}', '\u{2d7f}', Tifinagh), ('\u{2d80}', '\u{2d96}',
        Ethiopic), ('\u{2da0}', '\u{2da6}', Ethiopic), ('\u{2da8}', '\u{2dae}', Ethiopic),
        ('\u{2db0}', '\u{2db6}', Ethiopic), ('\u{2db8}', '\u{2dbe}', Ethiopic), ('\u{2dc0}',
        '\u{2dc6}', Ethiopic), ('\u{2dc8}', '\u{2dce}', Ethiopic), ('\u{2dd0}', '\u{2dd6}',
        Ethiopic), ('\u{2dd8}', '\u{2dde}', Ethiopic), ('\u{2de0}', '\u{2dff}', Cyrillic),
        ('\u{2e00}', '\u{2e42}', Common), ('\u{2e80}', '\u{2e99}', Han), ('\u{2e9b}', '\u{2ef3}',
        Han), ('\u{2f00}', '\u{2fd5}', Han), ('\u{2ff0}', '\u{2ffb}', Common), ('\u{3000}',
        '\u{3004}', Common), ('\u{3005}', '\u{3005}', Han), ('\u{3006}', '\u{3006}', Common),
        ('\u{3007}', '\u{3007}', Han), ('\u{3008}', '\u{3020}', Common), ('\u{3021}', '\u{3029}',
        Han), ('\u{302a}', '\u{302d}', Inherited), ('\u{302e}', '\u{302f}', Hangul), ('\u{3030}',
        '\u{3037}', Common), ('\u{3038}', '\u{303b}', Han), ('\u{303c}', '\u{303f}', Common),
        ('\u{3041}', '\u{3096}', Hiragana), ('\u{3099}', '\u{309a}', Inherited), ('\u{309b}',
        '\u{309c}', Common), ('\u{309d}', '\u{309f}', Hiragana), ('\u{30a0}', '\u{30a0}', Common),
        ('\u{30a1}', '\u{30fa}', Katakana), ('\u{30fb}', '\u{30fc}', Common), ('\u{30fd}',
        '\u{30ff}', Katakana), ('\u{3105}', '\u{312d}', Bopomofo), ('\u{3131}', '\u{318e}', Hangul),
        ('\u{3190}', '\u{319f}', Common), ('\u{31a0}', '\u{31ba}', Bopomofo), ('\u{31c0}',
        '\u{31e3}', Common), ('\u{31f0}', '\u{31ff}', Katakana), ('\u{3200}', '\u{321e}', Hangul),
        ('\u{3220}', '\u{325f}', Common), ('\u{3260}', '\u{327e}', Hangul), ('\u{327f}', '\u{32cf}',
        Common), ('\u{32d0}', '\u{32fe}', Katakana), ('\u{3300}', '\u{3357}', Katakana),
        ('\u{3358}', '\u{33ff}', Common), ('\u{3400}', '\u{4db5}', Han), ('\u{4dc0}', '\u{4dff}',
        Common), ('\u{4e00}', '\u{9fd5}', Han), ('\u{a000}', '\u{a48c}', Yi), ('\u{a490}',
        '\u{a4c6}', Yi), ('\u{a4d0}', '\u{a4ff}', Lisu), ('\u{a500}', '\u{a62b}', Vai), ('\u{a640}',
        '\u{a69f}', Cyrillic), ('\u{a6a0}', '\u{a6f7}', Bamum), ('\u{a700}', '\u{a721}', Common),
        ('\u{a722}', '\u{a787}', Latin), ('\u{a788}', '\u{a78a}', Common), ('\u{a78b}', '\u{a7ad}',
        Latin), ('\u{a7b0}', '\u{a7b7}', Latin), ('\u{a7f7}', '\u{a7ff}', Latin), ('\u{a800}',
        '\u{a82b}', Syloti_Nagri), ('\u{a830}', '\u{a839}', Common), ('\u{a840}', '\u{a877}',
        Phags_Pa), ('\u{a880}', '\u{a8c4}', Saurashtra), ('\u{a8ce}', '\u{a8d9}', Saurashtra),
        ('\u{a8e0}', '\u{a8fd}', Devanagari), ('\u{a900}', '\u{a92d}', Kayah_Li), ('\u{a92e}',
        '\u{a92e}', Common), ('\u{a92f}', '\u{a92f}', Kayah_Li), ('\u{a930}', '\u{a953}', Rejang),
        ('\u{a95f}', '\u{a95f}', Rejang), ('\u{a960}', '\u{a97c}', Hangul), ('\u{a980}', '\u{a9cd}',
        Javanese), ('\u{a9cf}', '\u{a9cf}', Common), ('\u{a9d0}', '\u{a9d9}', Javanese),
        ('\u{a9de}', '\u{a9df}', Javanese), ('\u{a9e0}', '\u{a9fe}', Myanmar), ('\u{aa00}',
        '\u{aa36}', Cham), ('\u{aa40}', '\u{aa4d}', Cham), ('\u{aa50}', '\u{aa59}', Cham),
        ('\u{aa5c}', '\u{aa5f}', Cham), ('\u{aa60}', '\u{aa7f}', Myanmar), ('\u{aa80}', '\u{aac2}',
        Tai_Viet), ('\u{aadb}', '\u{aadf}', Tai_Viet), ('\u{aae0}', '\u{aaf6}', Meetei_Mayek),
        ('\u{ab01}', '\u{ab06}', Ethiopic), ('\u{ab09}', '\u{ab0e}', Ethiopic), ('\u{ab11}',
        '\u{ab16}', Ethiopic), ('\u{ab20}', '\u{ab26}', Ethiopic), ('\u{ab28}', '\u{ab2e}',
        Ethiopic), ('\u{ab30}', '\u{ab5a}', Latin), ('\u{ab5b}', '\u{ab5b}', Common), ('\u{ab5c}',
        '\u{ab64}', Latin), ('\u{ab65}', '\u{ab65}', Greek), ('\u{ab70}', '\u{abbf}', Cherokee),
        ('\u{abc0}', '\u{abed}', Meetei_Mayek), ('\u{abf0}', '\u{abf9}', Meetei_Mayek), ('\u{ac00}',
        '\u{d7a3}', Hangul), ('\u{d7b0}', '\u{d7c6}', Hangul), ('\u{d7cb}', '\u{d7fb}', Hangul),
        ('\u{f900}', '\u{fa6d}', Han), ('\u{fa70}', '\u{fad9}', Han), ('\u{fb00}', '\u{fb06}',
        Latin), ('\u{fb13}', '\u{fb17}', Armenian), ('\u{fb1d}', '\u{fb36}', Hebrew), ('\u{fb38}',
        '\u{fb3c}', Hebrew), ('\u{fb3e}', '\u{fb3e}', Hebrew), ('\u{fb40}', '\u{fb41}', Hebrew),
        ('\u{fb43}', '\u{fb44}', Hebrew), ('\u{fb46}', '\u{fb4f}', Hebrew), ('\u{fb50}', '\u{fbc1}',
        Arabic), ('\u{fbd3}', '\u{fd3d}', Arabic), ('\u{fd3e}', '\u{fd3f}', Common), ('\u{fd50}',
        '\u{fd8f}', Arabic), ('\u{fd92}', '\u{fdc7}', Arabic), ('\u{fdf0}', '\u{fdfd}', Arabic),
        ('\u{fe00}', '\u{fe0f}', Inherited), ('\u{fe10}', '\u{fe19}', Common), ('\u{fe20}',
        '\u{fe2d}', Inherited), ('\u{fe2e}', '\u{fe2f}', Cyrillic), ('\u{fe30}', '\u{fe52}',
        Common), ('\u{fe54}', '\u{fe66}', Common), ('\u{fe68}', '\u{fe6b}', Common), ('\u{fe70}',
        '\u{fe74}', Arabic), ('\u{fe76}', '\u{fefc}', Arabic), ('\u{feff}', '\u{feff}', Common),
        ('\u{ff01}', '\u{ff20}', Common), ('\u{ff21}', '\u{ff3a}', Latin), ('\u{ff3b}', '\u{ff40}',
        Common), ('\u{ff41}', '\u{ff5a}', Latin), ('\u{ff5b}', '\u{ff65}', Common), ('\u{ff66}',
        '\u{ff6f}', Katakana), ('\u{ff70}', '\u{ff70}', Common), ('\u{ff71}', '\u{ff9d}', Katakana),
        ('\u{ff9e}', '\u{ff9f}', Common), ('\u{ffa0}', '\u{ffbe}', Hangul), ('\u{ffc2}', '\u{ffc7}',
        Hangul), ('\u{ffca}', '\u{ffcf}', Hangul), ('\u{ffd2}', '\u{ffd7}', Hangul), ('\u{ffda}',
        '\u{ffdc}', Hangul), ('\u{ffe0}', '\u{ffe6}', Common), ('\u{ffe8}', '\u{ffee}', Common),
        ('\u{fff9}', '\u{fffd}', Common), ('\u{10000}', '\u{1000b}', Linear_B), ('\u{1000d}',
        '\u{10026}', Linear_B), ('\u{10028}', '\u{1003a}', Linear_B), ('\u{1003c}', '\u{1003d}',
        Linear_B), ('\u{1003f}', '\u{1004d}', Linear_B), ('\u{10050}', '\u{1005d}', Linear_B),
        ('\u{10080}', '\u{100fa}', Linear_B), ('\u{10100}', '\u{10102}', Common), ('\u{10107}',
        '\u{10133}', Common), ('\u{10137}', '\u{1013f}', Common), ('\u{10140}', '\u{1018c}', Greek),
        ('\u{10190}', '\u{1019b}', Common), ('\u{101a0}', '\u{101a0}', Greek), ('\u{101d0}',
        '\u{101fc}', Common), ('\u{101fd}', '\u{101fd}', Inherited), ('\u{10280}', '\u{1029c}',
        Lycian), ('\u{102a0}', '\u{102d0}', Carian), ('\u{102e0}', '\u{102e0}', Inherited),
        ('\u{102e1}', '\u{102fb}', Common), ('\u{10300}', '\u{10323}', Old_Italic), ('\u{10330}',
        '\u{1034a}', Gothic), ('\u{10350}', '\u{1037a}', Old_Permic), ('\u{10380}', '\u{1039d}',
        Ugaritic), ('\u{1039f}', '\u{1039f}', Ugaritic), ('\u{103a0}', '\u{103c3}', Old_Persian),
        ('\u{103c8}', '\u{103d5}', Old_Persian), ('\u{10400}', '\u{1044f}', Deseret), ('\u{10450}',
        '\u{1047f}', Shavian), ('\u{10480}', '\u{1049d}', Osmanya), ('\u{104a0}', '\u{104a9}',
        Osmanya), ('\u{10500}', '\u{10527}', Elbasan), ('\u{10530}', '\u{10563}',
        Caucasian_Albanian), ('\u{1056f}', '\u{1056f}', Caucasian_Albanian), ('\u{10600}',
        '\u{10736}', Linear_A), ('\u{10740}', '\u{10755}', Linear_A), ('\u{10760}', '\u{10767}',
        Linear_A), ('\u{10800}', '\u{10805}', Cypriot), ('\u{10808}', '\u{10808}', Cypriot),
        ('\u{1080a}', '\u{10835}', Cypriot), ('\u{10837}', '\u{10838}', Cypriot), ('\u{1083c}',
        '\u{1083c}', Cypriot), ('\u{1083f}', '\u{1083f}', Cypriot), ('\u{10840}', '\u{10855}',
        Imperial_Aramaic), ('\u{10857}', '\u{1085f}', Imperial_Aramaic), ('\u{10860}', '\u{1087f}',
        Palmyrene), ('\u{10880}', '\u{1089e}', Nabataean), ('\u{108a7}', '\u{108af}', Nabataean),
        ('\u{108e0}', '\u{108f2}', Hatran), ('\u{108f4}', '\u{108f5}', Hatran), ('\u{108fb}',
        '\u{108ff}', Hatran), ('\u{10900}', '\u{1091b}', Phoenician), ('\u{1091f}', '\u{1091f}',
        Phoenician), ('\u{10920}', '\u{10939}', Lydian), ('\u{1093f}', '\u{1093f}', Lydian),
        ('\u{10980}', '\u{1099f}', Meroitic_Hieroglyphs), ('\u{109a0}', '\u{109b7}',
        Meroitic_Cursive), ('\u{109bc}', '\u{109cf}', Meroitic_Cursive), ('\u{109d2}', '\u{109ff}',
        Meroitic_Cursive), ('\u{10a00}', '\u{10a03}', Kharoshthi), ('\u{10a05}', '\u{10a06}',
        Kharoshthi), ('\u{10a0c}', '\u{10a13}', Kharoshthi), ('\u{10a15}', '\u{10a17}', Kharoshthi),
        ('\u{10a19}', '\u{10a33}', Kharoshthi), ('\u{10a38}', '\u{10a3a}', Kharoshthi),
        ('\u{10a3f}', '\u{10a47}', Kharoshthi), ('\u{10a50}', '\u{10a58}', Kharoshthi),
        ('\u{10a60}', '\u{10a7f}', Old_South_Arabian), ('\u{10a80}', '\u{10a9f}',
        Old_North_Arabian), ('\u{10ac0}', '\u{10ae6}', Manichaean), ('\u{10aeb}', '\u{10af6}',
        Manichaean), ('\u{10b00}', '\u{10b35}', Avestan), ('\u{10b39}', '\u{10b3f}', Avestan),
        ('\u{10b40}', '\u{10b55}', Inscriptional_Parthian), ('\u{10b58}', '\u{10b5f}',
        Inscriptional_Parthian), ('\u{10b60}', '\u{10b72}', Inscriptional_Pahlavi), ('\u{10b78}',
        '\u{10b7f}', Inscriptional_Pahlavi), ('\u{10b80}', '\u{10b91}', Psalter_Pahlavi),
        ('\u{10b99}', '\u{10b9c}', Psalter_Pahlavi), ('\u{10ba9}', '\u{10baf}', Psalter_Pahlavi),
        ('\u{10c00}', '\u{10c48}', Old_Turkic), ('\u{10c80}', '\u{10cb2}', Old_Hungarian),
        ('\u{10cc0}', '\u{10cf2}', Old_Hungarian), ('\u{10cfa}', '\u{10cff}', Old_Hungarian),
        ('\u{10e60}', '\u{10e7e}', Arabic), ('\u{11000}', '\u{1104d}', Brahmi), ('\u{11052}',
        '\u{1106f}', Brahmi), ('\u{1107f}', '\u{1107f}', Brahmi), ('\u{11080}', '\u{110c1}',
        Kaithi), ('\u{110d0}', '\u{110e8}', Sora_Sompeng), ('\u{110f0}', '\u{110f9}', Sora_Sompeng),
        ('\u{11100}', '\u{11134}', Chakma), ('\u{11136}', '\u{11143}', Chakma), ('\u{11150}',
        '\u{11176}', Mahajani), ('\u{11180}', '\u{111cd}', Sharada), ('\u{111d0}', '\u{111df}',
        Sharada), ('\u{111e1}', '\u{111f4}', Sinhala), ('\u{11200}', '\u{11211}', Khojki),
        ('\u{11213}', '\u{1123d}', Khojki), ('\u{11280}', '\u{11286}', Multani), ('\u{11288}',
        '\u{11288}', Multani), ('\u{1128a}', '\u{1128d}', Multani), ('\u{1128f}', '\u{1129d}',
        Multani), ('\u{1129f}', '\u{112a9}', Multani), ('\u{112b0}', '\u{112ea}', Khudawadi),
        ('\u{112f0}', '\u{112f9}', Khudawadi), ('\u{11300}', '\u{11303}', Grantha), ('\u{11305}',
        '\u{1130c}', Grantha), ('\u{1130f}', '\u{11310}', Grantha), ('\u{11313}', '\u{11328}',
        Grantha), ('\u{1132a}', '\u{11330}', Grantha), ('\u{11332}', '\u{11333}', Grantha),
        ('\u{11335}', '\u{11339}', Grantha), ('\u{1133c}', '\u{11344}', Grantha), ('\u{11347}',
        '\u{11348}', Grantha), ('\u{1134b}', '\u{1134d}', Grantha), ('\u{11350}', '\u{11350}',
        Grantha), ('\u{11357}', '\u{11357}', Grantha), ('\u{1135d}', '\u{11363}', Grantha),
        ('\u{11366}', '\u{1136c}', Grantha), ('\u{11370}', '\u{11374}', Grantha), ('\u{11480}',
        '\u{114c7}', Tirhuta), ('\u{114d0}', '\u{114d9}', Tirhuta), ('\u{11580}', '\u{115b5}',
        Siddham), ('\u{115b8}', '\u{115dd}', Siddham), ('\u{11600}', '\u{11644}', Modi),
        ('\u{11650}', '\u{11659}', Modi), ('\u{11680}', '\u{116b7}', Takri), ('\u{116c0}',
        '\u{116c9}', Takri), ('\u{11700}', '\u{11719}', Ahom), ('\u{1171d}', '\u{1172b}', Ahom),
        ('\u{11730}', '\u{1173f}', Ahom), ('\u{118a0}', '\u{118f2}', Warang_Citi), ('\u{118ff}',
        '\u{118ff}', Warang_Citi), ('\u{11ac0}', '\u{11af8}', Pau_Cin_Hau), ('\u{12000}',
        '\u{12399}', Cuneiform), ('\u{12400}', '\u{1246e}', Cuneiform), ('\u{12470}', '\u{12474}',
        Cuneiform), ('\u{12480}', '\u{12543}', Cuneiform), ('\u{13000}', '\u{1342e}',
        Egyptian_Hieroglyphs), ('\u{14400}', '\u{14646}', Anatolian_Hieroglyphs), ('\u{16800}',
        '\u{16a38}', Bamum), ('\u{16a40}', '\u{16a5e}', Mro), ('\u{16a60}', '\u{16a69}', Mro),
        ('\u{16a6e}', '\u{16a6f}', Mro), ('\u{16ad0}', '\u{16aed}', Bassa_Vah), ('\u{16af0}',
        '\u{16af5}', Bassa_Vah), ('\u{16b00}', '\u{16b45}', Pahawh_Hmong), ('\u{16b50}',
        '\u{16b59}', Pahawh_Hmong), ('\u{16b5b}', '\u{16b61}', Pahawh_Hmong), ('\u{16b63}',
        '\u{16b77}', Pahawh_Hmong), ('\u{16b7d}', '\u{16b8f}', Pahawh_Hmong), ('\u{16f00}',
        '\u{16f44}', Miao), ('\u{16f50}', '\u{16f7e}', Miao), ('\u{16f8f}', '\u{16f9f}', Miao),
        ('\u{1b000}', '\u{1b000}', Katakana), ('\u{1b001}', '\u{1b001}', Hiragana), ('\u{1bc00}',
        '\u{1bc6a}', Duployan), ('\u{1bc70}', '\u{1bc7c}', Duployan), ('\u{1bc80}', '\u{1bc88}',
        Duployan), ('\u{1bc90}', '\u{1bc99}', Duployan), ('\u{1bc9c}', '\u{1bc9f}', Duployan),
        ('\u{1bca0}', '\u{1bca3}', Common), ('\u{1d000}', '\u{1d0f5}', Common), ('\u{1d100}',
        '\u{1d126}', Common), ('\u{1d129}', '\u{1d166}', Common), ('\u{1d167}', '\u{1d169}',
        Inherited), ('\u{1d16a}', '\u{1d17a}', Common), ('\u{1d17b}', '\u{1d182}', Inherited),
        ('\u{1d183}', '\u{1d184}', Common), ('\u{1d185}', '\u{1d18b}', Inherited), ('\u{1d18c}',
        '\u{1d1a9}', Common), ('\u{1d1aa}', '\u{1d1ad}', Inherited), ('\u{1d1ae}', '\u{1d1e8}',
        Common), ('\u{1d200}', '\u{1d245}', Greek), ('\u{1d300}', '\u{1d356}', Common),
        ('\u{1d360}', '\u{1d371}', Common), ('\u{1d400}', '\u{1d454}', Common), ('\u{1d456}',
        '\u{1d49c}', Common), ('\u{1d49e}', '\u{1d49f}', Common), ('\u{1d4a2}', '\u{1d4a2}',
        Common), ('\u{1d4a5}', '\u{1d4a6}', Common), ('\u{1d4a9}', '\u{1d4ac}', Common),
        ('\u{1d4ae}', '\u{1d4b9}', Common), ('\u{1d4bb}', '\u{1d4bb}', Common), ('\u{1d4bd}',
        '\u{1d4c3}', Common), ('\u{1d4c5}', '\u{1d505}', Common), ('\u{1d507}', '\u{1d50a}',
        Common), ('\u{1d50d}', '\u{1d514}', Common), ('\u{1d516}', '\u{1d51c}', Common),
        ('\u{1d51e}', '\u{1d539}', Common), ('\u{1d53b}', '\u{1d53e}', Common), ('\u{1d540}',
        '\u{1d544}', Common), ('\u{1d546}', '\u{1d546}', Common), ('\u{1d54a}', '\u{1d550}',
        Common), ('\u{1d552}', '\u{1d6a5}', Common), ('\u{1d6a8}', '\u{1d7cb}', Common),
        ('\u{1d7ce}', '\u{1d7ff}', Common), ('\u{1d800}', '\u{1da8b}', SignWriting), ('\u{1da9b}',
        '\u{1da9f}', SignWriting), ('\u{1daa1}', '\u{1daaf}', SignWriting), ('\u{1e800}',
        '\u{1e8c4}', Mende_Kikakui), ('\u{1e8c7}', '\u{1e8d6}', Mende_Kikakui), ('\u{1ee00}',
        '\u{1ee03}', Arabic), ('\u{1ee05}', '\u{1ee1f}', Arabic), ('\u{1ee21}', '\u{1ee22}',
        Arabic), ('\u{1ee24}', '\u{1ee24}', Arabic), ('\u{1ee27}', '\u{1ee27}', Arabic),
        ('\u{1ee29}', '\u{1ee32}', Arabic), ('\u{1ee34}', '\u{1ee37}', Arabic), ('\u{1ee39}',
        '\u{1ee39}', Arabic), ('\u{1ee3b}', '\u{1ee3b}', Arabic), ('\u{1ee42}', '\u{1ee42}',
        Arabic), ('\u{1ee47}', '\u{1ee47}', Arabic), ('\u{1ee49}', '\u{1ee49}', Arabic),
        ('\u{1ee4b}', '\u{1ee4b}', Arabic), ('\u{1ee4d}', '\u{1ee4f}', Arabic), ('\u{1ee51}',
        '\u{1ee52}', Arabic), ('\u{1ee54}', '\u{1ee54}', Arabic), ('\u{1ee57}', '\u{1ee57}',
        Arabic), ('\u{1ee59}', '\u{1ee59}', Arabic), ('\u{1ee5b}', '\u{1ee5b}', Arabic),
        ('\u{1ee5d}', '\u{1ee5d}', Arabic), ('\u{1ee5f}', '\u{1ee5f}', Arabic), ('\u{1ee61}',
        '\u{1ee62}', Arabic), ('\u{1ee64}', '\u{1ee64}', Arabic), ('\u{1ee67}', '\u{1ee6a}',
        Arabic), ('\u{1ee6c}', '\u{1ee72}', Arabic), ('\u{1ee74}', '\u{1ee77}', Arabic),
        ('\u{1ee79}', '\u{1ee7c}', Arabic), ('\u{1ee7e}', '\u{1ee7e}', Arabic), ('\u{1ee80}',
        '\u{1ee89}', Arabic), ('\u{1ee8b}', '\u{1ee9b}', Arabic), ('\u{1eea1}', '\u{1eea3}',
        Arabic), ('\u{1eea5}', '\u{1eea9}', Arabic), ('\u{1eeab}', '\u{1eebb}', Arabic),
        ('\u{1eef0}', '\u{1eef1}', Arabic), ('\u{1f000}', '\u{1f02b}', Common), ('\u{1f030}',
        '\u{1f093}', Common), ('\u{1f0a0}', '\u{1f0ae}', Common), ('\u{1f0b1}', '\u{1f0bf}',
        Common), ('\u{1f0c1}', '\u{1f0cf}', Common), ('\u{1f0d1}', '\u{1f0f5}', Common),
        ('\u{1f100}', '\u{1f10c}', Common), ('\u{1f110}', '\u{1f12e}', Common), ('\u{1f130}',
        '\u{1f16b}', Common), ('\u{1f170}', '\u{1f19a}', Common), ('\u{1f1e6}', '\u{1f1ff}',
        Common), ('\u{1f200}', '\u{1f200}', Hiragana), ('\u{1f201}', '\u{1f202}', Common),
        ('\u{1f210}', '\u{1f23a}', Common), ('\u{1f240}', '\u{1f248}', Common), ('\u{1f250}',
        '\u{1f251}', Common), ('\u{1f300}', '\u{1f579}', Common), ('\u{1f57b}', '\u{1f5a3}',
        Common), ('\u{1f5a5}', '\u{1f6d0}', Common), ('\u{1f6e0}', '\u{1f6ec}', Common),
        ('\u{1f6f0}', '\u{1f6f3}', Common), ('\u{1f700}', '\u{1f773}', Common), ('\u{1f780}',
        '\u{1f7d4}', Common), ('\u{1f800}', '\u{1f80b}', Common), ('\u{1f810}', '\u{1f847}',
        Common), ('\u{1f850}', '\u{1f859}', Common), ('\u{1f860}', '\u{1f887}', Common),
        ('\u{1f890}', '\u{1f8ad}', Common), ('\u{1f910}', '\u{1f918}', Common), ('\u{1f980}',
        '\u{1f984}', Common), ('\u{1f9c0}', '\u{1f9c0}', Common), ('\u{20000}', '\u{2a6d6}', Han),
        ('\u{2a700}', '\u{2b734}', Han), ('\u{2b740}', '\u{2b81d}', Han), ('\u{2b820}', '\u{2cea1}',
        Han), ('\u{2f800}', '\u{2fa1d}', Han), ('\u{e0001}', '\u{e0001}', Common), ('\u{e0020}',
        '\u{e007f}', Common), ('\u{e0100}', '\u{e01ef}', Inherited)
    ];

