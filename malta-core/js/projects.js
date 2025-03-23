document.getElementById('form').addEventListener('submit', async event => {
  event.preventDefault();
  const name = event.target.name.value;
  const estimate = (v => v === '' ? null : +v)(event.target.estimate.value);
  event.target.reset()
  let response = await fetch('/project', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ name, estimate })
  })
  if (response.ok) {
    console.log(await response.json());
    location.reload();
  } else {
    console.error('Error adding project:', response.statusText);
  }
});

// Establish WebSocket connection for live project updates.
// const ws = new WebSocket('project/ws');
// ws.onopen = () => console.log('Connected to Projects WebSocket');

// ws.onmessage = event => {
//   try {
//     console.log(event.data)
//   } catch (error) {
//     console.error('Error parsing projects update:', error);
//   }
// };

// ws.onerror = error => console.error('WebSocket error:', error);