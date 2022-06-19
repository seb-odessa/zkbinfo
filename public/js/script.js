async function requestNamesAsync(ids) {
    const url = "https://esi.evetech.net/latest/universe/names/?datasource=tranquility";
    const response = await fetch(url, {
        method: 'POST',
        mode: 'cors',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json'
        },
        redirect: 'follow',
        referrerPolicy: 'no-referrer',
        body: JSON.stringify(ids)
    });
    return await response.json();
}


async function get(url = '') {
    const response = await fetch(url);
    return await response.json();
}

function most_active(obj, count) {
    // return new Map(Object.entries(obj).sort((a, b) => b[1] - a[1]).slice(0, count));
    let obj_map = new Map(Object.entries(obj).sort((a, b) => b[1] - a[1]).slice(0, count));
    let map = new Map();
    obj_map.forEach((value, key) => { map.set(key, value) });
    return map;
}

function format(activities) {

    let html = [];
    html.push("<p>Wins (" + activities.wins.killmails.length + "): ");
    for (let i = 0; i < activities.wins.killmails.length; i++) {
        const id = activities.wins.killmails[i];
        html.push(`<a href="https://zkillboard.com/kill/${id}/">${id}</a> `);
    }
    html.push("</p>");
    html.push("<p>Total damage dealt: " + activities.wins.total_damage + "</p>");
    html.push("<div>Systems with most activities: ")
    const wins_solar_systems = most_active(activities.wins.solar_systems, 5);
    wins_solar_systems.forEach((count, system_id) => {
        html.push(`<div id="wins_${system_id}" div style="display: inline">*</div> `);
    });
    html.push("</div>");
    html.push("<div>Favorite ships: ");
    const wins_ships = most_active(activities.wins.ships, 5);
    wins_ships.forEach((count, ship_id) => {
        html.push(`<div id="wins_${ship_id}" div style="display: inline">*</div> `);
    });
    html.push("</div>");


    requestNamesAsync(Array.from(wins_solar_systems.keys()))
        .then(names => {
            names.forEach((obj) => {
                const id = obj.id;
                const name = obj.name;
                const count = wins_solar_systems.get(`${id}`);
                const href = `<a href="https://zkillboard.com/system/${id}/">${name} (${count})</a>`;
                const element = `wins_${id}`;
                document.getElementById(element).innerHTML = href;
            });
        });

    requestNamesAsync(Array.from(wins_ships.keys()))
        .then(names => {
            names.forEach((obj) => {
                const id = obj.id;
                const name = obj.name;
                const count = wins_ships.get(`${id}`);
                const href = `<a href="https://zkillboard.com/ship/${id}/">${name} (${count})</a>`;
                const element = `wins_${id}`;
                document.getElementById(element).innerHTML = href;
            });
        });
    return html.join("");
}

function format_wins(activities) {

}