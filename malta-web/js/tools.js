document.getElementById('form').addEventListener('submit', async event => {
  event.preventDefault();
  const name = event.target.name.value.trim();
  const adjectives = event.target.adjectives.value.split(',').map(s => s.trim()).filter(Boolean);
  let response = await fetch('/tool', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ name, adjectives })
  })
  if (response.ok) {
    console.log(await response.json());
    event.target.reset()
    location.reload();
  } else {
    console.error('Error adding project:', response.statusText);
  }
});
