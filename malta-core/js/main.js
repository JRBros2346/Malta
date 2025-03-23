document.getElementById('income').addEventListener('submit', async event => {
    event.preventDefault();
    const amount = event.target.amount.value;
    const source = event.target.source.value;
    const date = event.target.date.value;
    event.target.reset()
    let response = await fetch('/income', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ amount, source, date })
    })
    if (response.ok) {
        console.log(await response.json());
        location.reload();
    } else {
        console.error('Error adding income:', response.statusText);
    }
});

document.getElementById('expense').addEventListener('submit', async event => {
    event.preventDefault();
    const amount = event.target.amount.value;
    const reason = event.target.reason.value;
    const date = event.target.date.value;
    event.target.reset()
    let response = await fetch('/income', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ amount, reason, date })
    })
    if (response.ok) {
        console.log(await response.json());
        location.reload();
    } else {
        console.error('Error adding income:', response.statusText);
    }
});