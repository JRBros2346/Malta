document.addEventListener("DOMContentLoaded", function () {
  // Assume the employee ID is provided as a query parameter (e.g., ?id=456)
  const urlParams = new URLSearchParams(window.location.search);
  const employeeId = urlParams.get("id");

  // Fetch employee details via AJAX (not implemented here)

  // Connect to the employee's WebSocket for updates (GET /employee/:id/ws)
  if (employeeId) {
    const ws = new WebSocket(`ws://${window.location.host}/employee/${employeeId}/ws`);
    ws.onmessage = function (event) {
      console.log("Update for employee", employeeId, ":", event.data);
      // Update the page as necessary.
    };
  }
});
