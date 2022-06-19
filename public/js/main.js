// document.getElementById("demo").innerHTML = "Hello from JavaScript";


const activity = {
    "id": 2115038999,
    "wins": {
        "killmails": [101469879, 101470695, 101470699, 101470696, 101470682, 101470701, 101507008, 101507542, 101309066, 101309064, 101524372, 101524708, 101526271, 101121954, 101121988, 100924854, 100925298, 100919451, 100919402, 100919875, 101123109, 100919483, 100919469, 100920381, 100946618, 100971335, 100968550, 100968576, 100957130, 101000216],
        "total_damage": 1392545,
        "ships": { "24702": 1, "3756": 8, "28659": 4, "49710": 1, "12038": 3, "22456": 11, "17918": 1 },
        "solar_systems": { "30001446": 1, "30000863": 1, "30000209": 1, "30002007": 1, "30002898": 1, "30000225": 1, "30000268": 1, "31001894": 1, "30000895": 1, "30000252": 5, "31002322": 3, "30000205": 1, "30000240": 1, "30000214": 1, "30001445": 2, "30000208": 4, "30000867": 1, "30002428": 2, "30002005": 1 }
    },
    "losses": {
        "killmails": [101487388, 101487418, 101518800, 101518813, 101116878, 100968687, 100968602],
        "total_damage": 114905,
        "ships": { "47466": 2, "670": 4, "22456": 1 },
        "solar_systems": { "30000611": 2, "30004218": 2, "30001768": 1, "31000005": 2 }
    }
};


function most_valuable_systems(systems, count) {
    const map = new Map(Object.entries(systems).sort((a, b) => b[1] - a[1]).slice(0, count));

    return map;
}

const obj = activity.wins.solar_systems;

const map = most_valuable_systems(obj, 5);
const ids = Array.from(map.keys());
console.log(ids);

for (const iteration of map) {
    // console.log(iteration.toString())
}


for (const iteration of map) {
    // <a href="https://developer.mozilla.org">MDN</a>
    const system_id = iteration[0];
    const msg = `https://zkillboard.com/system/${system_id}/`;
    // console.log(msg);
}

let url = "https://esi.evetech.net/latest/alliances/98676166/?datasource=tranquility";


const request = async() => {
    const response = await fetch(url);
    const json = await response.json();
    console.log(json);
}

const useRequest = async() => {
    const res = await request()
}

useRequest();