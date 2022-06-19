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

function format(prefix, systems, ships, damage) {
    let html = [];
    // html.push("<p>(" + report.killmails.length + "): ");
    // for (let i = 0; i < report.killmails.length; i++) {
    //     const id = report.killmails[i];
    //     html.push(`<a href="https://zkillboard.com/kill/${id}/">${id}</a> `);
    // }
    // html.push("</p>");
    html.push("<p>Total damage: " + damage + "</p>");
    html.push("<div>Systems with most activities: ")
    systems.forEach((count, system_id) => {
        html.push(`<div id="${prefix}_${system_id}" div style="display: inline">*</div> `);
    });
    html.push("</div>");

    html.push("<div>Favorite ships: ");
    ships.forEach((count, ship_id) => {
        html.push(`<div id="${prefix}_${ship_id}" div style="display: inline">*</div> `);
    });
    html.push("</div>");

    return html.join("");
}