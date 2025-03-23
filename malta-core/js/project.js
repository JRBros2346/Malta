document.addEventListener("DOMContentLoaded", function () {
  // Assume the project ID is passed via a query parameter (e.g., ?id=123)
  const urlParams = new URLSearchParams(window.location.search);
  const projectId = urlParams.get("id");

  // Fetch and display project details via AJAX (not implemented here)

  // Connect to the project's WebSocket for real-time updates (GET /project/:id/ws)
  if (projectId) {
    const ws = new WebSocket(`ws://${window.location.host}/project/${projectId}/ws`);
    ws.onmessage = function (event) {
      console.log("Update for project", projectId, ":", event.data);
      // Update the page with new data as necessary.
    };
  }
});
