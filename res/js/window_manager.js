globalThis.desktop_windows = []

// Handle when windows exit the windows_container when resising the screen
window.addEventListener("resize", () => {
  for (let i = 0; i < desktop_windows.length; i++) {
    let desktop_window = desktop_windows[i].window
    
    if (window.innerHeight * 0.80 > window.innerWidth) {
      desktop_window.classList.add("full")
      desktop_window.object.makeUndraggable()
      
      continue // Dont calculate clamping when they are .full, to save performance (Plus fixes some bugs!)
    } else {
      desktop_window.classList.remove("full")
      desktop_window.object.makeDraggable()
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
  constructor(data) {
    if (!data) throw TypeError("Argument 'data' must be defined!")
    
    if (!data.name) throw TypeError("Argument 'data' must have 'name' attribute!")
    if (!data.url) throw TypeError("Argument 'data' must have 'url' attribute!")
    if (!data.icon) throw TypeError("Argument 'data' must have 'icon' attribute!")
    // There's an optional 'short_name' attribute that will be used instead of 'name' for the apps_launcher
    
    const desktop_window = document.createElement("desktop_window")
    const app_icon = document.createElement("icon")
    
    const windows_container = document.getElementById("windows_container")
    const apps_launcher = document.getElementById("apps_launcher")
    
    const window_template = `
      <topbar class="font-bold">
        <div><div id="window_refresh"></div></div>
        <p>${data.name}</p>
        <img id="window_close" src="/res/img/window_close.svg">
      </topbar>
      <iframe id="app_container" src="${data.url}"></iframe>
    `
    
    desktop_window.className = "desktop_window"
    desktop_window.innerHTML = window_template
    
    app_icon.innerHTML = `<img src="${data.icon}" style="width: 5vh; height: 5vh"><div>`
    app_icon.onclick = () => {
      if (this.focused) this.hide()
      else this.focus()
    }
    apps_launcher.appendChild(app_icon)
    
    if (apps_launcher.childElementCount == 1) {
      apps_launcher.style.animation = ""
    }
    
    windows_container.appendChild(desktop_window)
    desktop_windows.push(this)
    
    desktop_window.object = this // So this class object can be accessed through the desktop_window
    this.window = desktop_window
    this.url = data.url
    this.icon = app_icon
    
    this.makeDraggable(windows_container)
    this.focus(desktop_window)
    
    // Detect those pesky phone screens. Harr!
    if (window.innerHeight * 0.80 > window.innerWidth) {
      desktop_window.classList.add("full")
      this.makeUndraggable()
    }
    
    function windowControlsHandler() {
      function handleControls(e) {
        if (startX == this.style.left && startY == this.style.top) {
          const target = e.target.id // We only care about the ID!
          
          if (target == "window_refresh") this.object.refresh()
          if (target == "window_close") this.object.hide()
        }
        
        this.removeEventListener("mouseup", handleControls)
        this.removeEventListener("touchend", handleControls)
      }
      
      this.object.focus() // Always focus
      
      let startX = this.style.left
      let startY = this.style.top
      
      this.addEventListener("mouseup", handleControls, { passive: true })
      this.addEventListener("touchend", handleControls, { passive: true })
    }
    
    desktop_window.addEventListener("mousedown", windowControlsHandler, { passive: true })
    desktop_window.addEventListener("touchstart", windowControlsHandler, { passive: true })
    
    // Center new windows
    //desktop_window.style.left = `${(windows_container.clientWidth / 2) - (window.innerHeight * 0.80 / 2) + ((desktop_windows.length - 1) * 30)}px`
    //desktop_window.style.top = `${(windows_container.clientHeight / 2) - (window.innerHeight * 0.50 / 2) + ((desktop_windows.length - 1) * 30)}px`
    
    desktop_window.style.left = `${(desktop_windows.length - 1) * 39}px`
    desktop_window.style.top = `${(desktop_windows.length - 1) * 39}px`
    
    /*return new Proxy(this, {
      get: (target, prop) => {
        if (!target.window) {
          throw new Error(`This DesktopWindow instance has been deleted!`);
        }
        return target[prop];
      }
    });*/
  }
  
  focus() {
    if (this.focused) return
    const index = desktop_windows.indexOf(this)
    
    desktop_windows.splice(index, 1)
    desktop_windows.push(this)
    
    this.hidden = false
    this.window.style.animation = ""

    for (let i = 0; i < desktop_windows.length; i++) {
      const desktop_window = desktop_windows[i].window
      
      desktop_window.style.zIndex = i
      desktop_window.children[1].style.pointerEvents = "none"
      
      if (i != desktop_windows.length - 1) {
        desktop_windows[i].focused = false
        desktop_window.style.filter = "brightness(0.6)"
        desktop_window.style.transition = "filter 0.4s"
        continue
      }
      
      desktop_windows[i].focused = true
      desktop_window.style.filter = "brightness(1)"
      desktop_window.style.transition = ""
      
      desktop_window.children[1].style.pointerEvents = ""
    }
  }
  
  refresh() {
    const refresh_icon = this.window.querySelector("#window_refresh")
    
    refresh_icon.style.animation = "refresh-animation 1s ease-out"
    setTimeout(() => {
      refresh_icon.style.animation = ""
    }, 1000)
    
    this.window.children[1].contentWindow.location.reload(true) // Reload iframe
  }
  
  hide() {
    this.window.style.animation = "0.2s forwards window-hide"
    this.focused = false
    this.hidden = true
    
    // Recalculate window focus
    let index = desktop_windows.indexOf(this) - 1
    
    if (desktop_windows.at(-1) != this) {
      index = -1
    }
    
    const target = desktop_windows.at(index)
    if (!target.hidden) target.focus()
  }
  
  delete() {
    this.hide() // Play hide animation / Handle focus recalulation
    this.icon.style.animation = "0.2s forwards icon-hide"
    
    if (this.icon.parentElement.childElementCount <= 1) {
      this.icon.parentElement.style.animation = "0.2s forwards icon-hide"
    }
    
    setTimeout(() => {
      const index = desktop_windows.indexOf(this)
      
      this.window.remove()
      this.icon.remove()
      
      desktop_windows.splice(index, 1)
      this.window = null
    }, 200)
  }
  
  makeUndraggable() { this.draggable = false }
  
  /* HACK HOUSE 2.0 */
  makeDraggable(container = this.container) {
    if (!container) throw Error("Argument 'container' must be defined at least once per window.")
    
    const desktop_window = this.window
    const topbar = desktop_window.querySelector("topbar")

    let offsetX, offsetY;

    function startDrag(e) {
      e.preventDefault()
      
      if (!this.parentElement.object.draggable) return

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
      e.preventDefault()
      handleMove(e.clientX, e.clientY)
    }
    function onTouchMove(e) {
      e.preventDefault()
      const touch = e.touches[0]
      handleMove(touch.clientX, touch.clientY)
    }

    function handleMove(clientX, clientY) {
      // Disable pointer events on the iframe
      desktop_window.children[1].style.pointerEvents = "none"
      
      const containerRect = container.getBoundingClientRect()
      const windowRect = desktop_window.getBoundingClientRect()

      let left = clientX - offsetX - containerRect.left
      let top = clientY - offsetY - containerRect.top
      
      // Window position limits calulations
      let windowLimits = windowRect.width / 3
      const windowLimitsMargin = 30
      
      // Check if window is inside container
      if (parseInt(desktop_window.style.left) + windowRect.width <= containerRect.width && parseInt(desktop_window.style.left) >= 0) {
        windowLimits = windowRect.width
        
        // Check if un-clamped position is greater than the margins, if it is then unlock window position (to 1/3)
        if ((left + windowRect.width - windowLimitsMargin) >= containerRect.width || (left + windowLimitsMargin) <= 0) {
          windowLimits = windowRect.width / 3
        }
      }
      // --
      
      // Constrain the window within the desktop, allowing overflow on the left and right,
      // While preventing overflow on the top and bottom.
      left = Math.max(-windowRect.width + windowLimits, Math.min(left, containerRect.width - windowLimits))
      top = Math.max(0, Math.min(top, containerRect.height - windowRect.height))

      desktop_window.style.left = `${left}px`
      desktop_window.style.top = `${top}px`
    }
    function stopDrag(e) {
      e.preventDefault()
      
      document.removeEventListener("mousemove", onMouseMove)
      document.removeEventListener("mouseup", stopDrag)

      document.removeEventListener("touchmove", onTouchMove)
      document.removeEventListener("touchend", stopDrag)
      
      desktop_window.children[1].style.pointerEvents = ""
    }
    
    topbar.addEventListener("mousedown", startDrag)
    topbar.addEventListener("touchstart", startDrag)
    
    this.container = container
    this.draggable = true
  }
}
