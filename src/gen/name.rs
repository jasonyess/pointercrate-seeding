use rand::{rngs::ThreadRng, Rng};

// These components are randomly joined to form "names"
const NAME_COMPONENTS: [&str; 400] = [
    "zor", "lek", "mir", "val", "dur", "tal", "zan", "reth", "kai", "vor", "lin", "dra", "shi",
    "mel", "fen", "rak", "tor", "el", "gai", "nyx", "sol", "ul", "drae", "quor", "xan", "neph",
    "jor", "vel", "cal", "syr", "ith", "myn", "zar", "eph", "nor", "thal", "ren", "vex", "kael",
    "bryn", "os", "sil", "ur", "ver", "nas", "rix", "del", "keir", "yra", "nex", "ar", "lua",
    "sed", "thy", "oza", "velk", "sym", "traz", "aen", "drez", "shor", "vo", "krai", "zen", "eth",
    "gal", "mora", "keth", "laz", "tes", "ora", "drax", "phyr", "tur", "vael", "nys", "kira",
    "ophel", "rum", "xyr", "rath", "jae", "zon", "mar", "seph", "quint", "yul", "torv", "vryn",
    "zeth", "thea", "nal", "durz", "hyl", "phae", "kar", "brel", "jen", "vox", "syl", "garn",
    "kae", "uln", "rad", "maer", "roth", "zev", "korr", "bel", "lom", "draq", "ser", "kral", "ael",
    "mal", "nar", "vorn", "xan", "kel", "rael", "fae", "tir", "ghul", "zorv", "mek", "lor", "zenk",
    "shi", "tan", "nys", "mirk", "jun", "zhor", "hael", "velm", "jaq", "cor", "tirn", "gai",
    "lazr", "seth", "vol", "ris", "glyn", "dar", "myn", "peir", "zorin", "krez", "ohr", "rel",
    "qan", "zem", "tri", "valk", "phir", "dan", "bren", "ahr", "tarn", "quell", "sola", "zarn",
    "vorn", "nur", "krae", "ysh", "thek", "an", "ior", "xul", "zhal", "kaor", "melk", "teth",
    "vun", "yrel", "thor", "zith", "yra", "dyrr", "grel", "morr", "carn", "ven", "drek", "jorl",
    "qen", "zhem", "alt", "brak", "lyss", "rell", "zarv", "skan", "quor", "tarn", "zir", "volm",
    "dusk", "aeth", "noir", "korr", "thun", "zer", "valm", "ruk", "ter", "ynor", "vohr", "pral",
    "zehn", "murn", "phaz", "quaz", "joth", "sael", "daen", "yr", "zanr", "rev", "kaeth", "rhun",
    "nira", "ull", "verr", "jyn", "yorn", "kair", "myrr", "dol", "siln", "zark", "tyr", "harn",
    "rhaz", "ulm", "zenr", "mekh", "torm", "brelk", "haen", "gryn", "orr", "qar", "veth", "yel",
    "qor", "pyrr", "braz", "ethr", "vehr", "lirr", "mer", "zaen", "qeir", "korm", "senn", "vozz",
    "zul", "ranz", "thae", "wyrr", "grae", "lorr", "rumk", "xer", "pan", "tra", "snor", "drae",
    "carn", "lurk", "drell", "phol", "kell", "morv", "kae", "venn", "noz", "raek", "zeln", "korr",
    "vask", "thez", "muld", "yaer", "selk", "nahl", "draz", "thry", "urn", "jarn", "vurn", "xerr",
    "farn", "yrr", "kaem", "syrn", "jyl", "zorh", "nezz", "thrak", "laek", "oll", "trem", "vhoz",
    "barl", "ghaz", "dyrl", "thex", "grelk", "marz", "zyrr", "pael", "rin", "fael", "muth", "brax",
    "ezz", "zern", "gael", "hul", "qarz", "shek", "derm", "ziro", "xel", "zulk", "lazr", "tanr",
    "reth", "synn", "phal", "myz", "draem", "taz", "thym", "gorn", "zuhl", "kralz", "venor",
    "thorm", "lair", "gnol", "sarn", "drekk", "zair", "murz", "kalr", "valth", "shor", "grim",
    "elz", "zur", "tov", "frel", "lorn", "saem", "torr", "melr", "jorm", "nell", "tair", "farn",
    "galz", "nyr", "phex", "zemm", "zarl", "dral", "thael", "kaem", "vroz", "sarx", "yx", "phor",
    "jerz", "sark", "rhae", "myrn", "xark", "ghael", "rell", "juun", "zurk", "vozz", "marn",
    "zarh", "kelz", "korrh", "rynn", "grael", "khel", "qarn", "shemm", "vaez", "lur", "drezz",
    "pal",
];

pub fn generate_member_name(rng: &mut ThreadRng) -> String {
    (0..rng.gen_range(2..4))
        .map(|_| NAME_COMPONENTS[rng.gen_range(0..NAME_COMPONENTS.len())])
        .collect()
}

pub fn generate_player_name(rng: &mut ThreadRng) -> String {
    generate_member_name(rng)
}

pub fn generate_demon_name(rng: &mut ThreadRng) -> String {
    let result: String = (0..rng.gen_range(2..4))
        .map(|_| {
            NAME_COMPONENTS[rng.gen_range(0..NAME_COMPONENTS.len())].to_owned()
                + rng.gen_ratio(1, 3).then(|| " ").unwrap_or("")
        })
        .collect();

    rng.gen_ratio(1, 3)
        .then(|| result.to_uppercase())
        .unwrap_or_else(|| result.clone())
}
