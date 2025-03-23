document.getElementById('income').addEventListener('submit', async event => {
  event.preventDefault();
  const amount = +event.target.amount.value.trim();
  const id = event.target.id.value.trim();
  const on_date = (v => v ? new Date(v) : new Date())(event.target.date.value).toJSON();
  let response = await fetch(`/project/${id}/income`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ amount, on_date })
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
  const id = event.target.id.value.trim();
  const reason = event.target.reason.value.trim();
  const on_date = (v => v ? new Date(v) : new Date())(event.target.date.value).toJSON();
  let response = await fetch(`/project/${id}/expense`, {
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