document.getElementById('form').addEventListener('submit', async event => {
  event.preventDefault();
  const name = event.target.name.value;
  event.target.reset()
  let response = await fetch('/employee', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ name })
  })
  if (response.ok) {
    console.log(await response.json());
    location.reload();
  } else {
    console.error('Error adding employee:', response.statusText);
  }
})