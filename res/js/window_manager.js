globalThis.desktop_windows = []

// Handle when windows exit the apps_container when resising the screen
window.addEventListener("resize", () => {
  for (let i = 0; i < desktop_windows.length; i++) {
    let desktop_window = desktop_windows[i]
    
    let containerRect = desktop_window.parentElement.getBoundingClientRect()
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

class DesktopWindow {
  constructor(data, id = "") {
    let desktop_window = document.createElement("desktop_window")
    let apps_container = document.getElementById("apps_container")
    let topbar = document.createElement("topbar")
    
    desktop_window.id = id
    desktop_window.class = "desktop_window"
    
    desktop_window.style.height = "50vh" // Former: 22vh
    desktop_window.style.width = "80vh" // Former: 30vh
    desktop_window.style.display = "block"
    desktop_window.style.background = "white"
    desktop_window.style.position = "absolute"
    desktop_window.style.borderRadius = "1.3vh"
    desktop_window.style.overflow = "hidden"
    
    topbar.style.height = "8%" // Former: 2vh
    topbar.style.width = "100%"
    topbar.style.display = "block"
    topbar.style.background = "red"
    
    desktop_window.appendChild(topbar)
    this.makeDraggable(desktop_window, apps_container)
    
    desktop_window.deleteWindow = this.deleteWindow
    window.moveToTop = this.moveToTop
    
    apps_container.appendChild(desktop_window)
    desktop_windows.push(desktop_window)
    
    desktop_window.addEventListener("mousedown", window.moveToTop)
    desktop_window.addEventListener("touchstart", window.moveToTop)
    window.moveToTop()
    
    
    console.log(desktop_windows)
    return desktop_window
  }
  
  moveToTop() {
    let index = desktop_windows.indexOf(this)
    
    if (index > -1) {
      desktop_windows.splice(index, 1)
      desktop_windows.push(this)
    }
    
    for (let i = 0; i < desktop_windows.length; i++) {
      desktop_windows[i].style.zIndex = i
      desktop_windows[i].style.filter = "brightness(1)"
      
      if (i != desktop_windows.length - 1) {
        desktop_windows[i].style.filter = "brightness(0.6)"
      }
    }
  }
  
  deleteWindow() {
    let index = desktop_windows.indexOf(this)
    
    desktop_windows.splice(index, 1)
    this.parentElement.removeChild(this)
  }
  
  /* HACK HOUSE 2 */
  makeDraggable(desktop_window, container) {
    const topbar = desktop_window.querySelector("topbar")

    let offsetX, offsetY;

    function startDrag(e) {
      e.preventDefault()

      if (e.type === "touchstart") {
        // For touch events
        offsetX = e.changedTouches[0].clientX - topbar.getBoundingClientRect().left
        offsetY = e.changedTouches[0].clientY - topbar.getBoundingClientRect().top
      } else {
        if (e.which == 3) return // Prevent drag on right click
        
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
      const containerRect = container.getBoundingClientRect()
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
}
