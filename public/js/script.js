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

function sort_by_count(obj) {
    // let obj_map = new Map(Object.entries(obj).sort((a, b) => b[1] - a[1]).slice(0, count));
    let obj_map = new Map(Object.entries(obj).sort((a, b) => b[1] - a[1]));
    let map = new Map();
    obj_map.forEach((value, key) => { map.set(key, value) });
    return map;
}

function make_damage(damage) {
    return "<p>Total damage: " + damage + "</p>";
}

function make_items(msg, prefix, map, display = 6) {
    let html = [];
    html.push("<div>" + msg + ":&nbsp;");
    let idx = 0;
    map.forEach((count, id) => {
        html.push(`<div id="${prefix}_${id}" div style="display: inline">*</div> `);
        if (display == ++idx && idx != map.size) {
            html.push("<details><summary>More (" + (map.size - idx) + ") items...</summary>");
        } else if (idx == map.size) {
            html.push("</details>");
        }
    });
    html.push("</div>");
    return html.join("");
}

function update(category, prefix, names, map) {
    names.forEach((obj) => {
        const id = obj.id;
        const name = obj.name;
        const count = map.get(`${id}`);
        const href = `<a href="/gui/${category}/${name}/">${name} (${count})</a>`;
        const element = `${prefix}_${id}`;
        document.getElementById(element).innerHTML = href;
    });
}


function draw_prime_time(hourly) {
    const canvas = document.getElementById('prime_time').getContext('2d');
    const data = {
        datasets: [{
            label: 'killmails/hour',
            data: hourly,
            backgroundColor: 'rgba(255, 99, 132, 0.2)',
            borderColor: 'rgba(255, 99, 132, 1)',
            borderWidth: 1
        }]
    };

    const config = {
        type: 'bar',
        data: data,
        options: {
            responsive: false,
            scales: {
                y: {
                    beginAtZero: true
                }
            }
        }
    };

    const myChart = new Chart(canvas, config);
}