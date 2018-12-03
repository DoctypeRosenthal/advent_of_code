use std::str::Chars;

type HasDouble = bool;
type HasTriple = bool;


fn find_times(needle: char, haystack: &str, exact_occurence: u16) -> bool {
    let mut counter:u16 = 0;
    for c in haystack.chars() {
        if c == needle { counter += 1 }
    }

    if counter == exact_occurence {
        return true
    }

    return false
}

fn check_multiples(mut needle_iter: Chars, haystack: &str, exact_occurence: u16) -> bool {
    match needle_iter.next() {
        None => return false,
        Some(needle) => find_times(needle, haystack, exact_occurence) || check_multiples(needle_iter, haystack, exact_occurence)
    }
}

fn check_for_multiples(s: &str) -> (HasDouble, HasTriple) {
    (check_multiples(s.chars(), s, 2), check_multiples(s.chars(), s, 3))
}

fn multiply_shit(v: Vec<(HasDouble, HasTriple)>) -> i32 {
    let (a, b) = v
        .iter()
        .fold((0, 0), |(doubles, triples), (has_double, has_triple)| (if *has_double {doubles+1} else {doubles}, if *has_triple {triples+1} else {triples}));
    a*b
}
use utils::*;

pub fn main() {
    let str_part1 = split_by_newline(
        "icxjubroqtunzeyzpomfksahgw
ibxjvbroqtunleyzjdmfksahow
icxjvbroqtinleyzpdmflsahnw
icxjvbnoqtunleyvpgmfksahgw
wcxjvbroqrunleyzpdmfksahge
icxjtbroqtjzleyzpdmfksahgw
icxjvbrohtunleyzpdmfkbahsw
xcxjvbroqcunleyzpdmfksaxgw
ycxjvbroqtunleyzpowfksahgw
icxfvbroqtunleyzpdmfksncgw
ixxjvbuoqtunleyzpdvfksahgw
icfjvbroqtunleyzpdmfksadgt
icxjvbroqdunleyzpdafksqhgw
icxjvbrobtunlelzpdmfkuahgs
ujxjvbroqtunleyzpdmqksahgw
icqjvsroqtunleyzpdmfksahuw
icxjvbroptpnleyzpdmfksangw
ipxjvbroqtunleyzpdmfesahgi
icajvbroqtunltyzpdqfksahgw
ickjvbroqtuzleyzpdmfgsahgw
icxjvbroqtunledzpdmwksahgz
icxjvlroqtsnleyzpdmfksvhgw
icxjvbroqtunleyzpdsfysahvw
icxjvbroqtunwnyzydmfksahgw
ionjvbroqtunleyzpdmfksahgj
icxjvwriqtunleyzpdmfksahgi
ocxjvbroztunleyzpdmfksapgw
icxjvbroqtmnlewzpumfksahgw
ucxjvbroqtunleyzpdmzktahgw
icxgvbroqtunleyztdmfktahgw
icxhvbroqttnleybpdmfksahgw
icxjvbroqtugleyzpdxfkeahgw
acxjvbroqvunlexzpdmfksahgw
icxjvbroqglnleyzpbmfksahgw
icxjvbriqtnvleyzpdmfksahgw
icxjvbreqtunlakzpdmfksahgw
gcxjvbuoqtunleyzpdmfksawgw
icxjvbroqtunleyzpddfkyzhgw
icxjvbjoqtunleyzpdmfqsahhw
icxjvjroqtunleyzpnmfksajgw
ycxjvbroqtunmeyzcdmfksahgw
irxkvbryqtunleyzpdmfksahgw
isxjvbrlqtunleyzpdmsksahgw
icxjvbcoqtunleyzpdfkksahgw
ixnjvbroqtunleyzpdmfkqahgw
wcxjvbroqhunleyzqdmfksahgw
icljvurmqtunleyzpdmfksahgw
ibxjvbroqtayleyzpdmfksahgw
arxjvbroqiunleyzpdmfksahgw
iuxjvbroqtunluyzpdmoksahgw
icxjvbrmqtunleyzpdmfosahew
isxjvbroqtunleyrpdmfksrhgw
icxjvxrpqtunleyzpdmfkdahgw
ichjvbrogtunllyzpdmfksahgw
icxjvbeoqtunlryzpdmfksakgw
icxjvbroqtcnemyzpdmfksahgw
icxjvbroqtybledzpdmfksahgw
icxjvbrqqtunleyzpdmfksgngw
icgjvbroqtunleyzmdmfksabgw
icxjtbroqtunleyzkdmfksahww
icxjvbfoqtunleyzpamfqsahgw
icxjvbroknuyleyzpdmfksahgw
icxjvbroqtujleyzpdmaksaigw
icxjvbroqtnnlmyzpdmflsahgw
icxjvbroqtunlefzpdmfsfahgw
icxjvdroqtusleyzpdzfksahgw
icxjvbioqtunlsyzpdmfkshhgw
icxbvbrodtunleyzpdmoksahgw
icxjvbroqtuvleyzpdmfkbahmw
iyxjvbroqtunljvzpdmfksahgw
icxjvbroqtudleynddmfksahgw
icxjvwroqtnnleyzpdmfksahgz
ichjvbroqtunleyzjdmeksahgw
icxjvbrostunluyrpdmfksahgw
icfjvbroqtunleyxpdgfksahgw
nhxjvbroqtunlerzpdmfksahgw
icxjvbrothunlexzpdmfksahgw
icxjvbrzltqnleyzpdmfksahgw
icxjvbrhqtunleyzpdmfksajgy
vcxjvjroqiunleyzpdmfksahgw
icxjfbroltunleyzpdmqksahgw
icxbvbroqtunleyzpdofasahgw
icxjvbkoqtunveyzpdmfksaqgw
icxsebroqtunleyzpdmuksahgw
icxjvbroquunlpyrpdmfksahgw
icxhvbroqtunjeyzpdmrksahgw
icdjvbroqtunlzyzpdmfksangw
jcxqvbroqtvnleyzpdmfksahgw
icxjvxroqtunleyrpdmfxsahgw
icxjvnroqtunleyzpdmfssyhgw
icxjvbraptunleyzpdofksahgw
icxjvbroatunleyjpdmfbsahgw
icxjvbroytlnlryzpdmfksahgw
iaxjvbroqkunleyzpdmfcsahgw
ucxjvbroqtuileyzzdmfksahgw
icxjqbroqtcnleyzpgmfksahgw
icxjvbloqtunleyzadmfksaqgw
icxjvbroqtunleyzkdmnksakgw
icxjvbroqtunleyjpdxfksahvw
iqxjvbroqtujleyzpdmfklahgw
icgjvbroqtunleyzpdmfksbhgb
icxjzbroqtunleyzpdmfkdahgg
icxjvbrobtunloywpdmfksahgw
icxavbroqtunleyfpdmfksahgd
icxjvbroqtunleyophmfksahkw
icxjndroqtunlyyzpdmfksahgw
icxjvbroqtjnleyppdmvksahgw
icxjvbroonfnleyzpdmfksahgw
icxjvbrqqtlnljyzpdmfksahgw
icxjvbrzqtunlelspdmfksahgw
icxjvbooqtunleyztdmfkfahgw
icajvbroltunlnyzpdmfksahgw
icxjvbroqtunleyzidmdkschgw
icxjvbroktupleyzpdmfksahyw
icxjcbroyeunleyzpdmfksahgw
icxjvbroqtunlezkpdmfksahsw
icxjvbroqtunlejzpcmfksrhgw
icxjvvroqtunlsyzkdmfksahgw
icxjnbroqtunbeyzpdmfpsahgw
itxjbbroqtunleyzpemfksahgw
icxjvbroqtunlrkzpdmfksshgw
rcxjvbroqtujlbyzpdmfksahgw
icxjvmroqtugleazpdmfksahgw
icxjvbfoqtunleylpdmfkeahgw
icnjvoroktunleyzpdmfksahgw
icxjvbroqtunlvyzpdmfkfahgr
icxjvbroqtgnseyzpdmfxsahgw
scxjvbroqtunleycpdmfksdhgw
icxhvbxoqtunleuzpdmfksahgw
icxjvbruqtunleyzpnmfksfhgw
icdjvbroqtunleyzpdmfksahup
ihxjvbroqtunleovpdmfksahgw
icxjvbroqtunleyzxdmfksahzv
ocxjvbioqtunleyzpdmfzsahgw
idxjvbroqtunlyyzpdofksahgw
izdjvbroqtunleyzpdtfksahgw
icxjvbrnqtunleyzpdmfksbhgb
icxjvbrjqtunleyhpdmrksahgw
icxjvbroqtunleyzpdbflsahgg
icxjvbmfqtunleyzpdmfkaahgw
idxjvbroqtunlsyzpdffksahgw
bcxjvbroqtunleyzpkmfkswhgw
ivxjvbroqtdnleyzpdmbksahgw
icxpvbboqtunleyzpdmfksahtw
ibxjvbroqtunlehzpdmfkmahgw
icxjvbboqtunleyzpdmfkaahgv
icxjlaroqtuileyzpdmfksahgw
icxjvbroftunleyzpdmfqsahew
ichjvbroqtunleyzpdmiwsahgw
icxrvbvoqtunleyzpdmiksahgw
icxjvbroqtunldydpdmfksahgl
icogvbroqtunleyzpdmfnsahgw
icxjvbroqtunleszodmfkswhgw
icxjvbrontunleyztemfksahgw
icxjvbrovtunleyzpdvkksahgw
icxjvbroqqucteyzpdmfksahgw
icmovbroptunleyzpdmfksahgw
icxjvbqoftunleyzvdmfksahgw
icxjvbdoqtunleyzpdmfkadhgw
icxjvbroqtunlgnzpdmfksaqgw
icxjvbroqtunieygpdyfksahgw
acdjvbroqtunleyzpdmfkwahgw
icxjvbroqtunleyzpdmfkfahlj
icxjvbgoqtunleyepdmvksahgw
icxjvbpobbunleyzpdmfksahgw
icxjvbroqtunleurpdmfktahgw
ipxjvbzoqtunleyzpdmffsahgw
icxjtbroqtunllyzpdmuksahgw
icxjvbroqtunbsyzadmfksahgw
ihxjvoroqtuqleyzpdmfksahgw
idxjmbroqqunleyzpdmfksahgw
wcxjvbdoqtunleyzpdmfksahgr
icxjvbroqtunleygptmfksahgj
ipxjvbrsqtunleyzpdmfksghgw
ycxjvbroqtunluyzkdmfksahgw
icxjvbroxtuulejzpdmfksahgw
icqjvbroqtunlwyypdmfksahgw
ioxjhbroqtunleyzphmfksahgw
icxjvbgoqnunleyzpdmfksahaw
mcxjvbroqtunleyzpdmfksihgh
icxjsbroqtunlqyzpdmfksawgw
icxjvbroqtuoleycpdmftsahgw
icxjvbroqtunleyzgdifksahlw
icxjvbmoqtunleyzjfmfksahgw
icxjvbroqtunlezopdmfksahge
icxjvbroqtbnlefzpdmfosahgw
tcxjvbromtunlevzpdmfksahgw
irxjgbroqtunleyzpdmfksthgw
icxjvbrojtunleyxpdmoksahgw
icxrvbroytpnleyzpdmfksahgw
icxjvbroqtunfeyupdmfksasgw
ihqjvbroqtunleyzpdmftsahgw
icxjobroqkunleozpdmfksahgw
icjjjbroqtualeyzpdmfksahgw
icxjvbroqtunaeytpdmfksahvw
icxjvbroqtunzeyzpdmfkshhxw
icxqvbroqtucleyzxdmfksahgw
icxjvbrogturleyzxdmfksahgw
icxjvoqoqtunleyzpdcfksahgw
iuxjvbroqtunleyzpdmfksopgw
icxjveroqtunleyzptmfksalgw
icxjvbroqtunleyzpdmfwcahhw
iwxjvbroqtlnleyzpdmfksyhgw
ectjvbroqtanleyzpdmfksahgw
icxjvnroqtyhleyzpdmfksahgw
icvjvhboqtunleyzpdmfksahgw
icxjtbroqtuzleyupdmfksahgw
icjjvproqtunleyzpsmfksahgw
icdjvbroqtutleyzpdmiksahgw
icxjvwroqtujleyzpdmfksahgc
icxjxbroqtunleyzpdwhksahgw
icxjvbqoqtunleyzpdmvfsahgw
icajvbroqtusleyzpdmfksaagw
icxjvbroqtunbtyzpdmfksmhgw
kcxjvbroqtxnleyzpdmfkskhgw
icxjvbqogfunleyzpdmfksahgw
icxjvbroqtubleyzpdmfdswhgw
icxjvprlqtunleyzpdmffsahgw
icxjxbroqtucleyzpdmfksakgw
dcxrvbroqtunleycpdmfksahgw
icxjvbrobtunleyzpomfksahgu
ocxrvbroqtunleyzpdmfssahgw
icxjvbroktunlejzpdmfksahzw
icxjvbrovtunleyzmdmfkhahgw
icxjvbroqtudleygpdmfksfhgw
bcxjvbroqtubllyzpdmfksahgw
icxwvbrontunzeyzpdmfksahgw
icxjvbroqtunleysjbmfksahgw
icxjvvroztunleyzpdmfksjhgw
ivxjxbroqtunleyzpdmfksahew
icxjvbroqtunleyupqufksahgw
icxjvmrcqtunleyzpdmxksahgw
icxjvgroqtunleyzpdgfkuahgw
icxjvbroqthnqeyfpdmfksahgw
icxjsbuodtunleyzpdmfksahgw
iuxjzbroqtunleyzpdrfksahgw
icxjvbrobtunlelzpdmfksahgs
icxjvbroqtzhljyzpdmfksahgw
inxtvbroqtunleyzpdmeksahgw
icgjvbroqtunleyztdmfksahgq
icxjvagoqtugleyzpdmfksahgw
icxuvbroqtunleyzpimfkyahgw
icxzvbroqtfhleyzpdmfksahgw
icxjjbroqtqnleyzpdmnksahgw
icjrvbroqtunleszpdmfksahgw
iexjvbroqtunlgyzpdmfksacgw
rcxjvbkoqtuoleyzpdmfksahgw
icxjvbroqgunlwyzpdmfksqhgw
icxjvbroqtunleqzpsmfksqhgw
icxjvbroqtubaeyzpdmfksaugw"
    );
    println!("part 1: {}", multiply_shit(str_part1.iter().map(|&x| check_for_multiples(x)).collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        let examples = vec![
            ("abcdef", (false, false)),
            ("bababc", (true, true)),
            ("abbcde", (true, false)),
            ("abcccd", (false, true)),
            ("aabcdd", (true, false)),
            ("abcdee", (true, false)),
            ("ababab", (false, true))
        ];
        examples
            .iter()
            .map(|(question, result)| (check_for_multiples(&question), result))
            .for_each(|(question, &result)| assert_eq!(question, result));

        assert_eq!(
            multiply_shit(examples
                .iter()
                .map(|(_, result)| *result)
                .collect()
            ),
            12
        );
    }
}