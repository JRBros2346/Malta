document.getElementById('form').addEventListener('submit', async event => {
  event.preventDefault();
  const name = event.target.name.value;
  const adjectives = event.target.adjectives.value.split(',').map(s => s.trim()).filter(Boolean);
  event.target.reset()
  let response = await fetch('/tool', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ name, adjectives })
  })
  if (response.ok) {
    console.log(await response.json());
    location.reload();
  } else {
    console.error('Error adding project:', response.statusText);
  }
});
