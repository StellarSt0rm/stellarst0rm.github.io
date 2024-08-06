globalThis.desktop_windows = []

class DesktopWindow {
  constructor(id, data) {
    function makeDraggable(desktop_window, apps_container) {
      const topbar = desktop_window.querySelector("topbar")

      let offsetX, offsetY;

      function startDrag(e) {
        e.preventDefault()

        if (e.type === "touchstart") {
          // For touch events
          offsetX = e.changedTouches[0].clientX - topbar.getBoundingClientRect().left
          offsetY = e.changedTouches[0].clientY - topbar.getBoundingClientRect().top
        } else {
          // For mouse events
          offsetX = e.clientX - topbar.getBoundingClientRect().left
          offsetY = e.clientY - topbar.getBoundingClientRect().top
        }

        document.addEventListener("mousemove", onMouseMove)
        document.addEventListener("mouseup", stopDrag)

        document.addEventListener("touchmove", onTouchMove)
        document.addEventListener("touchend", stopDrag)
      }

      function onMouseMove(e) {
        handleMove(e.clientX, e.clientY)
      }
      function onTouchMove(e) {
        // Handle the first touch point
        const touch = e.touches[0]
        handleMove(touch.clientX, touch.clientY)
      }

      function handleMove(clientX, clientY) {
        const containerRect = apps_container.getBoundingClientRect()
        const windowRect = desktop_window.getBoundingClientRect()

        let left = clientX - offsetX - containerRect.left
        let top = clientY - offsetY - containerRect.top

        // Constrain the window within the desktop, allowing overflow on the left and right,
        // While preventing overflow on the top and bottom.
        left = Math.max(-windowRect.width + windowRect.width / 3, Math.min(left, containerRect.width - windowRect.width / 3))
        top = Math.max(0, Math.min(top, containerRect.height - windowRect.height))

        desktop_window.style.left = `${left}px`
        desktop_window.style.top = `${top}px`
      }
      function stopDrag() {
        document.removeEventListener("mousemove", onMouseMove)
        document.removeEventListener("mouseup", stopDrag)

        document.removeEventListener("touchmove", onTouchMove)
        document.removeEventListener("touchend", stopDrag)
      }

      topbar.addEventListener("mousedown", startDrag)
      topbar.addEventListener("touchstart", startDrag)
    }
    
    let desktop_window = document.createElement("desktop_window")
    let apps_container = document.getElementById("apps_container")
    let topbar = document.createElement("topbar")
    
    desktop_window.id = id
    desktop_window.class = "desktop_window"
    
    desktop_window.style.height = "22vh"
    desktop_window.style.width = "30vh"
    desktop_window.style.display = "block"
    desktop_window.style.background = "black"
    desktop_window.style.position = "absolute"
    
    topbar.style.height = "2vh"
    topbar.style.width = "30vh"
    topbar.style.display = "block"
    topbar.style.background = "red"
    
    desktop_window.appendChild(topbar)
    makeDraggable(desktop_window, apps_container)
    
    apps_container.appendChild(desktop_window)
    desktop_windows.push(desktop_window)
    
    console.log(desktop_windows)
    return desktop_window
  }
}

// Handle when windows exit the apps_container when resising the screen
window.addEventListener("resize", () => {
  for (let i = 0; i < desktop_windows.length; i++) {
    let desktop_window = desktop_windows[i]
    
    let containerRect = apps_container.getBoundingClientRect()
    let windowRect = desktop_window.getBoundingClientRect()
    
    let left = parseInt(desktop_window.style.left)
    let top = parseInt(desktop_window.style.top)
    
    left = Math.max(-windowRect.width + windowRect.width / 3, Math.min(left, containerRect.width - windowRect.width / 3)) 
    top = Math.max(0, Math.min(top, containerRect.height - windowRect.height))
    
    desktop_window.style.left = `${left}px`
    desktop_window.style.top = `${top}px`
  }
})

/*
let array = [1, 2, 3, 4, 5];
let valueToRemove = 3;
let index = array.indexOf(valueToRemove);

if (index > -1) {
    array.splice(index, 1);
}

console.log(array); // Output: [1, 2, 4, 5]
*/
