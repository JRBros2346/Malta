document.getElementById('form').addEventListener('submit', async event => {
  event.preventDefault();
  const name = event.target.name.value.trim();
  const estimate = (v => v ? null : +v)(event.target.estimate.value);
  let response = await fetch('/project', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ name, estimate })
  })
  if (response.ok) {
    console.log(await response.json());
    event.target.reset()
    location.reload();
  } else {
    console.error('Error adding project:', response.statusText);
  }
});
