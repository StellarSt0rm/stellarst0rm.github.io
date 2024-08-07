// TODO: Pass A LOT of styles to CSS, and CSS classes!

globalThis.desktop_windows = []

// Handle when windows exit the apps_container when resizing the screen
window.addEventListener("resize", () => {
  for (let i = 0; i < desktop_windows.length; i++) {
    let desktop_window = desktop_windows[i]
    
    if (window.innerHeight * 0.80 > window.innerWidth) {
      desktop_window.classList.add("full")
      desktop_window.makeUndraggable()
      
      continue // Dont calculate clamping when they are .full, to save performance (Plus fixes some bugs!)
    } else {
      desktop_window.classList.remove("full")
      desktop_window.makeDraggable()
    }
    
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

class DesktopWindow {
  constructor(data, id = "") {
    if (!data) throw Error("Argument 'data' must be defined!")
    
    const desktop_window = document.createElement("desktop_window")
    const topbar = document.createElement("topbar")
    const apps_container = document.getElementById("apps_container")
    
    if (id) desktop_window.id = id
    desktop_window.className = "desktop_window"
    
    desktop_window.deleteWindow = this.deleteWindow
    desktop_window.makeDraggable = this.makeDraggable
    desktop_window.makeUndraggable = this.makeUndraggable
    desktop_window.moveToTop = this.moveToTop
    
    desktop_window.appendChild(topbar)
    desktop_window.makeDraggable(apps_container)
    
    // Detect those pesky phone screens.
    if (window.innerHeight * 0.80 > window.innerWidth) {
      desktop_window.classList.add("full")
      desktop_window.makeUndraggable()
    }
    
    apps_container.appendChild(desktop_window)
    desktop_windows.push(desktop_window)
    
    desktop_window.addEventListener("mousedown", desktop_window.moveToTop, { passive: true })
    desktop_window.addEventListener("touchstart", desktop_window.moveToTop, { passive: true })
    desktop_window.moveToTop()
    
    return desktop_window
  }
  
  moveToTop() {
    const index = desktop_windows.indexOf(this)
    
    desktop_windows.splice(index, 1)
    desktop_windows.push(this)
    
    for (let i = 0; i < desktop_windows.length; i++) {
      desktop_windows[i].style.zIndex = i
      desktop_windows[i].style.filter = "brightness(1)"
      desktop_windows[i].style.transition = "none"
      
      if (i != desktop_windows.length - 1) {
        desktop_windows[i].style.filter = "brightness(0.6)"
        desktop_windows[i].style.transition = "filter 0.4s"
      }
    }
  }
  
  deleteWindow() {
    const index = desktop_windows.indexOf(this)
    
    desktop_windows.splice(index, 1)
    this.parentElement.removeChild(this)
    
    desktop_windows.at(-1).moveToTop() // Recalculate window focus
  }
  
  makeUndraggable() {
    this.desktop_draggable = false
  }
  
  /* HACK HOUSE 2 */
  makeDraggable(container = this.container) {
    if (!container) throw Error("Argument 'container' must be defined at least once per window.")
    
    const desktop_window = this
    const topbar = desktop_window.querySelector("topbar")

    let offsetX, offsetY;

    function startDrag(e) {
      e.preventDefault()
      
      if (!desktop_window.desktop_draggable) return

      if (e.type === "touchstart") {
        // For touch events
        offsetX = e.changedTouches[0].clientX - topbar.getBoundingClientRect().left
        offsetY = e.changedTouches[0].clientY - topbar.getBoundingClientRect().top
      } else {
        if (e.which != 1) return // Prevent drag on right/middle click
        
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
      e.preventDefault()
      
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
    function stopDrag(e) {
      document.removeEventListener("mousemove", onMouseMove)
      document.removeEventListener("mouseup", stopDrag)

      document.removeEventListener("touchmove", onTouchMove)
      document.removeEventListener("touchend", stopDrag)
    }
    
    topbar.addEventListener("mousedown", startDrag)
    topbar.addEventListener("touchstart", startDrag)
    
    desktop_window.container = container
    desktop_window.desktop_draggable = true
  }
}
