document.getElementById('income').addEventListener('submit', async event => {
    event.preventDefault();
    const amount = +event.target.amount.value.trim();
    const source = event.target.source.value.trim();
    const on_date = (v => v ? new Date(v) : new Date())(event.target.date.value).toJSON();
    console.log({ amount, source, on_date });
    let response = await fetch('/income', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ amount, source, on_date })
    })
    if (response.ok) {
        console.log(await response.json());
        event.target.reset()
        location.reload();
    } else {
        console.error('Error adding income:', response.statusText);
    }
});

document.getElementById('expense').addEventListener('submit', async event => {
    event.preventDefault();
    const amount = +event.target.amount.value.trim();
    const reason = event.target.reason.value.trim();
    const on_date = (v => v ? new Date(v) : new Date())(event.target.date.value).toJSON();
    console.log({ amount, reason, on_date });
    let response = await fetch('/expense', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ amount, reason, on_date })
    })
    if (response.ok) {
        console.log(await response.json());
        event.target.reset()
        location.reload();
    } else {
        console.error('Error adding income:', response.statusText);
    }
});