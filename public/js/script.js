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

function make_damage(damage) {
    return "<p>Total damage: " + damage + "</p>";
}

function make_items(msg, prefix, map) {
    let html = [];
    html.push(`<div>${msg}: `)
    map.forEach((count, id) => {
        html.push(`<div id="${prefix}_${id}" div style="display: inline">*</div> `);
    });
    html.push("</div>");
    return html.join("");
}

function format(prefix, systems, ships, damage) {
    let html = [];
    html.push(make_damage(damage));
    html.push(make_items("Systems with most activities", prefix, systems));
    html.push(make_items("Favorite ships", prefix, ships));
    return html.join("");
}