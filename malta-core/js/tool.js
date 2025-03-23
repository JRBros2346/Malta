document.addEventListener("DOMContentLoaded", function () {
  // Assume the tool ID is provided as a query parameter (e.g., ?id=789)
  const urlParams = new URLSearchParams(window.location.search);
  const toolId = urlParams.get("id");

  // Fetch tool details via AJAX (not implemented here)

  // Connect to the tool's WebSocket for updates (GET /tool/:id/ws)
  if (toolId) {
    const ws = new WebSocket(`ws://${window.location.host}/tool/${toolId}/ws`);
    ws.onmessage = function (event) {
      console.log("Update for tool", toolId, ":", event.data);
      // Update the page as needed.
    };
  }
});
