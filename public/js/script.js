async function postData(url = '', data = {}) {
    // Default options are marked with *
    const response = await fetch(url, {
        method: 'POST', // *GET, POST, PUT, DELETE, etc.
        mode: 'cors', // no-cors, *cors, same-origin
        cache: 'no-cache', // *default, no-cache, reload, force-cache, only-if-cached
        // credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/json'
                // 'Content-Type': 'application/x-www-form-urlencoded',
        },
        redirect: 'follow', // manual, *follow, error
        referrerPolicy: 'no-referrer', // no-referrer, *client
        body: JSON.stringify(data) // body data type must match "Content-Type" header
    });
    return await response.json(); // parses JSON response into native JavaScript objects
}

async function get(url = '') {
    const response = await fetch(url);
    return await response.json();
}

function most_active(obj, count) {
    const map = new Map(Object.entries(obj).sort((a, b) => b[1] - a[1]).slice(0, count));

    return map;
}

function most_active_systems(obj, count) {
    return Object.entries(obj).join(" ");
}

function most_active_ships(obj, count) {
    return Object.entries(obj).join(" ");
}

function format(activities) {
    let html = [];
    html.push("<p>Activity last 14 days:</p>");
    html.push("<p>Wins (" + activities.wins.killmails.length + "): ");
    for (let i = 0; i < activities.wins.killmails.length; i++) {
        const id = activities.wins.killmails[i];
        html.push(`<a href="https://zkillboard.com/kill/${id}/">${id}</a> `);
    }

    html.push("</p>");
    return html.join("");
}