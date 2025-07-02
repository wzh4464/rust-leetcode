pub struct Solution;

const MOD: i64 = 1_000_000_007;

fn modinv(a: i64, m: i64) -> i64 {
    let mut a = a;
    let mut m = m;
    let (mut x0, mut x1) = (0, 1);
    while a > 1 {
        let q = a / m;
        (a, m) = (m, a % m);
        (x0, x1) = (x1 - q * x0, x0);
    }
    if x1 < 0 {
        x1 += MOD;
    }
    x1
}

fn modfact(n: usize) -> Vec<i64> {
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }
    fact
}

fn modcomb(n: usize, k: usize, fact: &Vec<i64>) -> i64 {
    if k > n {
        return 0;
    }
    let numerator = fact[n];
    let denominator = fact[k] * fact[n - k] % MOD;
    numerator * modinv(denominator, MOD) % MOD
}

impl Solution {
    /// [Problem Number]. [Problem Title]
    ///
    /// [Problem Description]
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_leetcode::problems::[module_name]::Solution;
    /// assert_eq!(Solution::[method_name]([example_input]), [expected_output]);
    /// ```
    pub fn possible_string_count(word: String, k: i32) -> i32 {
        let (_, acc_vec) = word.chars().fold(
            (None, Vec::<i32>::new()),
            |(prev, mut acc_vec), ch| match prev {
                Some(p) if p == ch => (prev, {
                    *(acc_vec.last_mut().unwrap()) += 1;
                    acc_vec
                }),
                _ => (Some(ch), {
                    acc_vec.push(0);
                    acc_vec
                }),
            },
        );

        // dbg!(&acc_vec);

        let await_to_distribute = word.len() as i32 - k;

        let m = acc_vec.len();

        let await_vec: Vec<i32> = (0..=await_to_distribute).collect();

        let result = await_vec.iter().fold(0, |result: i32, &left| {
            let tmp = Solution::drawer(left, &acc_vec);
            dbg!(tmp, left);
            result + tmp
        });

        result

        // todo!("Implement the solution")
    }

    fn drawer(left: i32, acc_vec: &Vec<i32>) -> i32 {
        if left == 0 {
            1 as i32
        } else {
            
        }
        // todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        // Test case 1
        let word = "aabbccdd";
        let k = 7;
        assert_eq!(Solution::possible_string_count(word.to_string(), k), 5);
    }

    #[test]
    fn test_example_2() {
        // Test case 2
        let word = "nnnnuuuudppvvvrrrrrrrryyyfffffffooooooooooooooosssssssmmjjmmmrrrbbbbbbbbeqttmmmmmmmkkkkuuuuunnbbbbaaaaaaaaauuuuuuuuuaaaaawffffzzzzzffffffgggggyyyyyyyyyyyyyyyssbbbbbbbbbbfzzzffbbbiiiimmqqqquuuuuuuuuuzhhhhhhhhpppppppuuuuuuunnnnnnnyyyyyggggggrrrrrrzzzzzttttttttkkkkkkkvvvvvvvqqqqqqqwwwwooooooogggggggnnnnnnnlllkkkkkkkkkkiiiiinsssszzzzzzzzzzjfffddddeeeeiiiivvvvvvvvvffrrrrrrrzzrrrrrrrrrrxxxxxxbbbbbbbttddxxxxxiiiiiqqqqqqqqqyyyyyyyyyvvzlllllllllmmmmmzzzzzzztzzzzppqqqqqxxxxxxxxxxxddddddddddxxxxxxxxxsssssssstttlllkkkkkkaaaaaddddddlllllllllloooooorrrrrrrrrpppoooooooottttttttttuuuuuurqqmmmmmmmvjttttttttfffffffffffffffftttttttttnnnzzzvvttttaaaaaaaaammmmmmmmcccccccwzzzzzzzbrrrrrrrssssssssddddddbbiiiiiiiiqqqqqqehhhhhhccmmmmmmmmmaaaaaaaayyyyyyyyygggjjqqqqqqqssssqqqmmmmmmmmvvvvvvvvffggggggpppppppyyymmmmmmmmmkkkbbbbbbbbbbhhhhkkkkkkknnnnnnnnnnooooooddllllllllllvvvvvvvvqaarrrrrrnnnnnnnnhhmmmmmdddddddddkkknnnnnnnnnuuuuuuuuuuuuuuuuuuuuurrrrqqtttllllllliiiiizzzzuuuuuuuzzqqqqqqqqqkkkkkkkkkkiccciiiiiiiiiirrrrrreeeeeaaahhhhhhbbjjjjjkkkkkkkkkksssssttttttttttaaaahhuuppppppnnuuuuuuuufffffffffffkkksssssrrrddddddddddkkkkkkkkkfeeekkkkkkaaaaaaaaapppppppppsssssssssssssssszzzzzgggggggggzeeeeeeoooooooocccctuuuuaaaaaaannnnnnnnfuuqqqqqpppssssssssssffffffffzzzzzggggggggddddddddnddddddddddfffffffssssssseewwwbbbbbbbuuuuuccgguuuuuukkkkkkkkkkvvvvvvuuuuuuueeeeeeeeqqqqqqlllllllnnnnnhhhhhhhhffffqqqqqqqqqcccccccvvvvvvvvvvgggzzzzzzzkkkkkkkkkrriiiiiiieeeeemmmmmmmmmoohhdddddduuxxxxxxxqqqmmmmmmmmvvvvvvvhhhhhhhhhwwwwakkkkxxxxxxxxxkiiissppeeeqqqqbbbbbbbbfffffffddiiiiiiiyyyyhhhhhhhhhhmmmmmmmpppwwwwwwwwwceeeeeeeeenddwwwwwyccccceeeekkkkvvvvvffffffffttttddhhhhhhhhzzznrrrrrrrrrhhhhhhhhhhhccccccvvvvvvviiifffffffffgghhhhheehhhhhhhhhhmmmmmmkkkkkkkvvnnjjjjjjsssssssssvvvvvvvvuuuuccccccrrrrrrrrrrqqqqqqurrrrrrssssswwwwwhhbbbbbffffffffffaaaaaaaallcccccgqqqqqqqqqqrrrrrrrrrrvvvvviiiiiiiiiiqqqqqqqeeeeeeeeeeiiiiiiiiiidddeeeeeeeeeeoooooooooobbbbbbbbbbyyyyyyyyyaaaaaaaaahhjjjjjjjkkkkkkkkiiiiiiiiiidddffffffffdnnnnnnaaaazzzzzzrrrooooosssssjhhhhhhcwwwwwwwwwweeeeaaaaaaaaffffffrrrrrrmmmmrqqqqqqbbbjjjjjjjmmiiiisssssssaaaaaaavvvvvvvvnnnnnnnnnucccccccccccccccccccccccccccccccuuvvvvvvvwwwmmmmmmmmeeeeeeeeiiiiiiiueeeeeeeeeetttrrrrrrrrrppeeesiijggggggvvddxxxxxxxxxuuuuuuuuuaaaaaaaaaattttttttttmmeeeeeennnnooooopppppppaaaaaaaaaaiiiiiiiirrrrrrrrnnnnnnxxxxxxxxqqqqqqqqqjjjjjjppbbbbbbpppppppppllllllllzzzzzddddddddsssssssssooooooossssssssssrrrrkkkkkkkkkzzzzzzzzuuuuwwwwwwwxxxxxnnnnnnnnnnxrrrrrrrrruuuuupssssssqqqqppppppuuuuuuuuugggggyyyyynnnnnnnnnhhxxxxxxmmmmmmmmmmtttttttttccaaaaaqqqqqqqqqqaaaajhhhhhhhhhfffffzzzzzzzssssswwwwwwwxxxxxxxxxxgggggggfffffffffjjjjjbbbboooooooookkkkeeeeexxhiiiiiinnnnnnnnnnkkkkknnnnnnnnhhhhhhhhhhhhwwwwwwqqqqqqtttttttttzzzzwwwwwwwwwwffjjjjjjjjjsssssssssggggggggggeeeekkrrrrrrapppppaaaaaaaffffuuuuggggggggtttttttiiiiiiaaaaahhhhhhhhhsssssssssovwwwwwssssssssssjjjjjjjnhhhhhrrrrrrrrrrddmmcddddddddddttttttttttttttttttnnnnnnnnnzaaaaaagggggggghhhhhhhhmbbbbbaaaaaaalllwwwwwwwwmmmmmmeeeeeqmmmmmmmmmmrrrrhhhhoooooowwwwwwwwwmmmmmmmmmmvvvrrrrrrhhhhhhhhhuuuuuukkkkxxxxxwwwaaaaaaaaacccccddddppppppuuuuuuuupppppppfffffzzppzzzzzzzzzzrrrruuvvbbbbbbbbmmmmmmmmmmnnnnnnnnuuuuuuuusssmmmmmmmmmmiiiiiiieeeeeeeeccccvvvvvvvvvvkkkkkkkkggggbbnnnnddooooommqqqzzzzzzzpppgggggggooooooooooiiiiiiiiicccccccuuuuuuuuuugggggeeeeeeeuuuxxxxxxiiiiiiiddddffwwwwwwddddddbbbbbbbbbbbbbbbbbwwwwwjjjjjjjjyyyysssuuuuuuurrrrrrraaaaaaaannnnnnnnnttttwwwwxxxxbbbuuugnnnnnnnnnnrrrrrrbbbbbbbbbbbbkkkkkkkkkbbbbbbbbbbbjjjjjjjjjcccccccwwwwwwmmmooooozzzzzzzzgggggffffaddllllllllllyyyaaaaarrrrrrrrrrrrrrnnnnnnnnnnhhhhuuzzzzuuusssssssssxxxxxxxxxxlllllllllqqccccccccceeeeyyyyyyyyyyccccaaaaaaaaaazzzzzzzfbbbbbbboooooobbbbbbbnnnnnnnnxxxxzzuuuuuuuiijjjwwwwwwwwwxxqqqqqccccjuuuuuuummmmmmmmuuttttteeeeeiiiiinnnnnnnnnwwwwwwwkkkkkkkkkuuuuuuttttttllllllllllzziwwwmmmmmmmkkiiiiiiidddddddqqqqqqlllllllllddddddddrrrrrrreeeeeegggggggggfffffffrrrrrwwnnnnnnnnqqqqqqiiiimmmmmmmmmmmffffffffffiiiiiiiiiimmmmmmmtoooooooooottttttttdddddixxcccccccccoojjjjjjjjjffffffffffffffgggggggzzzzzoogggggiiiiiffffffffkvvvvvwwwuwwwwwwwzzzzzzzrrrrrrxxxxxxxxxxfhhhhhhhhhhqqqqqqqqqgggmmmmmmmmmppppppppppxxxxxvvvvvvvvvvlllllqqqqqqtttppppphhhhhhjjjjjjjjjjdddddrrrrrrggrrrrrraaaaaaaaammmmaaaaaaddwwqqwwwuubbbbbbbbevvvvrrrrrrrrrrddzzzzzzlllllllaaaaaaaabbbyyyyyyyyyylllllllllbbbbbboooooooodddddwwwwwwkkkkkkkkkkddddddhhhhhhhhhhccccccccsssaaeeqqqqqqqpppppcyyyyyyxxrhhqqqqqeeeeeppppphhhhdddddddffffmmmmgggggeeqqqmmmmmmmmmmvvnnnnnnnnuuuuuuuaaaaawwwwiiiiiiiiiiyyyyyyyyxxxxxxxxxxxxxvvvvvvddddddbbewpppppnnnnnniiiiirrrrrggggggghhhhhhhhhhaaaaxxxxxffffffjjjjwwwwwwwwyyyyyyymmmmrrrrryyyyjjjjvvvvvvvccyyccccciiiiiiirrrrrfffffffffffffiiiiiiiwwwwwwwwrrrrrgggrrrrrrrrvvvvuuuuuummmpppppppppjjjjjdddddddxxxxxxxxxxiiqqqqqqqkkkkkdddyyyyyyyyppppppppeeeettttnnnnnnnnnyyyyyyyyykkktttkkkkkkkkwwwwwrrrrnnnnnnnnnjjjjjddddddqqqqxxxxhhhhhhhhhwwwwwwzzzzzzzzzjjjjjjjjjjuuuhhhhhbyyyyyyyyvvvvvvvvvvvdddhhhhhhhhllllllppppppppxxfffffffjjjjkkkkkkkssooooooooouuuuuuiiiiigggggguppppcccccccccmmmmmmeeeeeeeejjjuuuuulllllllbbbbbbirrrrrrrriiiiiiiiiiuuuuuuuuuu";
        let k = 915;
        assert_eq!(Solution::possible_string_count(word.to_string(), k), 5);
        todo!("Add test case")
    }

    #[test]
    fn test_edge_case_empty() {
        // Edge case: empty input
        todo!("Add edge case test")
    }

    #[test]
    fn test_edge_case_single() {
        // Edge case: single element
        todo!("Add edge case test")
    }
}

#[cfg(test)]
mod bench {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_small_input(b: &mut Bencher) {
        // Small input benchmark
        todo!("Add benchmark")
    }

    #[bench]
    fn bench_medium_input(b: &mut Bencher) {
        // Medium input benchmark
        todo!("Add benchmark")
    }

    #[bench]
    fn bench_large_input(b: &mut Bencher) {
        // Large input benchmark
        todo!("Add benchmark")
    }
}
