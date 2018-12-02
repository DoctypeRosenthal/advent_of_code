fn char_to_int(x: char) -> u32 {
    x.to_digit(10).unwrap()
}

fn get_twins((a, b): (char, char)) -> Option<u32> {
    if a == b {
        Some(b.to_digit(10).unwrap())
    } else {
        None
    }
}

fn part1(x: &str) -> u32 {
    let sequence1 = x.to_string();
    let sequence2 = [&sequence1[1..], &sequence1[0..1]].concat();

    sequence1.chars().zip(sequence2.chars())
        .filter_map(get_twins)
        .sum::<u32>()
}

fn sum_twin_pairs((a, b): (char, char)) -> u32 {
    if a == b {
        char_to_int(a) + char_to_int(b)
    }
    else {
        0
    }
}

fn part2(x: &str) -> u32 {
    let sequence = x.to_string();
    let first_half = &sequence[..sequence.len()/2];
    let second_half = &sequence[sequence.len()/2..];

    first_half.chars().zip(second_half.chars())
        .map(sum_twin_pairs)
        .sum::<u32>()
}

pub fn main() {
    println!("part1: {}", part1(
             "4281224989975872839961169513979579335691369498483794171253625322698694611857431137339\
             9233137985644636248212964655628661154375656427571535987492489811342447278297478946434\
             8626278532936228881786273586278886575828239366794429223317476722337424399239986153675\
             27592411332256187381436445133918691881345168526319289162718676981812871559571544456544\
             45815146775218749359429135471217518516313733161224914715646977312989519895119172726843\
             35463436218283261962158671266625299188764589814518793576375629163896349665312991285776\
             59514214626179224447572178294136478796892453784169853828845935515978398563818725465385\
             18648745445848789991932426416118597567286346238534756384789237444715638456354681738241\
             96684361934269459459124269196811512927442662761563824323621758785866391424778683599179\
             44784559593192858925593595329511193743126681535278139996729538933962617866414841556117\
             53867259924697828887579425583621179386293691294397174274744168516281211916393556463942\
             76451847131182652486561415942815818785884559193483878139351841633366398788657844396925\
             42321766251735648619382134145488928326669122477872383339791422439672255959395912531717\
             58995946855248524194957933894818313547872874523671456618292875187716319393146831377224\
             93531318181315216342994141683484111969476952946378314883421677952397588613562958741328\
             98773456549237897739643148121598365681448651886564264561241394512948546497953599167577\
             63387867589971281246513111531828161889249351863618137972519976439926862947246992819694\
             73142721116432968216434977684138184481963845141486793996476793954226225885432422654394\
             43988284216329545854975513724761433899187996666592546654511189971494371657111332647943\
             29259392279967999512794857228367544577376681918459145667322859284537818187922364478161\
             27492445993945894435692799839217467253986218213131249786833333936332257795191937942688668182629489191693154184177398186462481316834678733713614889439352976144726162214648922159719979143735815478633912633185334529484779322818611438194522292278787653763328944421516569181178517915745625295158611636365253948455727653672922299582352766484"
    )); // -> 1034, correct! :)

    println!("part2: {}", part2(
        "428122498997587283996116951397957933569136949848379417125362532269869461185743113733992331379856446362482129646556286611543756564275715359874924898113424472782974789464348626278532936228881786273586278886575828239366794429223317476722337424399239986153675275924113322561873814364451339186918813451685263192891627186769818128715595715444565444581514677521874935942913547121751851631373316122491471564697731298951989511917272684335463436218283261962158671266625299188764589814518793576375629163896349665312991285776595142146261792244475721782941364787968924537841698538288459355159783985638187254653851864874544584878999193242641611859756728634623853475638478923744471563845635468173824196684361934269459459124269196811512927442662761563824323621758785866391424778683599179447845595931928589255935953295111937431266815352781399967295389339626178664148415561175386725992469782888757942558362117938629369129439717427474416851628121191639355646394276451847131182652486561415942815818785884559193483878139351841633366398788657844396925423217662517356486193821341454889283266691224778723833397914224396722559593959125317175899594685524852419495793389481831354787287452367145661829287518771631939314683137722493531318181315216342994141683484111969476952946378314883421677952397588613562958741328987734565492378977396431481215983656814486518865642645612413945129485464979535991675776338786758997128124651311153182816188924935186361813797251997643992686294724699281969473142721116432968216434977684138184481963845141486793996476793954226225885432422654394439882842163295458549755137247614338991879966665925466545111899714943716571113326479432925939227996799951279485722836754457737668191845914566732285928453781818792236447816127492445993945894435692799839217467253986218213131249786833333936332257795191937942688668182629489191693154184177398186462481316834678733713614889439352976144726162214648922159719979143735815478633912633185334529484779322818611438194522292278787653763328944421516569181178517915745625295158611636365253948455727653672922299582352766484"
        ) // 1356, correct! :)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_part1() {
        assert_eq!(part1("1122"), 3);
        assert_eq!(part1("1111"), 4);
        assert_eq!(part1("1234"), 0);
        assert_eq!(part1("91212129"), 9);
    }

    #[test]
    fn examples_part2() {
        assert_eq!(part2("1212"), 6);
        assert_eq!(part2("1221"), 0);
        assert_eq!(part2("123425"), 4);
        assert_eq!(part2("123123"), 12);
        assert_eq!(part2("12131415"), 4);
    }
}