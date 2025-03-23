document.getElementById('form').addEventListener('submit', async event => {
  event.preventDefault();
  const name = event.target.name.value.trim();
  let response = await fetch('/employee', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ name })
  })
  if (response.ok) {
    console.log(await response.json());
    event.target.reset()
    location.reload();
  } else {
    console.error('Error adding employee:', response.statusText);
  }
})