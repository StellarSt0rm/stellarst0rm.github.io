globalThis.notification_queue = []

class DesktopNotification {
  constructor(data) {
    if (!data) throw TypeError("Argument 'data' must be defined!")
    
    if (!data.app) throw TypeError("Argument 'data' must have 'app' attribute!")
    if (!data.text) throw TypeError("Argument 'data' must have 'text' attribute!")
    if (!data.image) throw TypeError("Argument 'data' must have 'image' attribute!")
    // There's an optional 'time' attribute that will be used for the notification's Timeout
    
    const notification_template = `
      <img id="notification_close" src="/res/img/window_close.svg">
      
      <div style="display: flex; align-items: center; margin: 1vh 0 0 1vh">
        <img src="/res/img/system_run.svg" style="height: 2.5vh">
        <p style="font-weight: bold; margin: 0">${data.app}</p>
        <p style="opacity: 0.6; font-size: 1.1vh; margin: 0 0 0 0.5vh">-&nbsp; Just now?</p>
      </div>
      <div style="display: flex; margin: 0.3vh 0 0 1vh">
        <img style="height: 5.5vh; margin-right: 1vh; border-radius: 2vh" src="${data.image}">
        <div style="max-height: 5.5vh; overflow: scroll; scrollbar-color: gray transparent; padding-right: 1vh; min-width: 35vh;">${parseBytedown(data.text)}</div>
      </div>
    `
    
    if (parseInt(data.time) == 0 || parseInt(data.time)) this.time = parseInt(data.time) * 1000
    else this.time = 5000
    
    this.template = notification_template
    notification_queue.push(this)
    
    if (notification_queue.length <= 1) this.show()
  }
  
  show() {
    if (notification_queue[0] != this) { notification_queue[0].delete(); return }
    
    const notification = document.getElementById("notification")
    
    notification.innerHTML = this.template
    notification.style.animation = "notification-show 0.3s forwards cubic-bezier(.18,.89,.32,1.28)"
    
    if (window.innerHeight * 0.52 > window.innerWidth) notification.classList.add("full")
    else notification.classList.remove("full")
    
    if (this.time) setTimeout(() => { this.delete() }, this.time)
    notification.onclick = () => { this.delete() }
  }
  
  delete() {
    const index = notification_queue.indexOf(this)
    if (index == -1) return // Return if the notification has already been deleted
    
    const notification = document.getElementById("notification")
    
    notification_queue.splice(index, 1)
    notification.style.animation = "notification-hide 0.3s cubic-bezier(.6,-0.28,.74,.05)"
    
    if (notification_queue[0]) setTimeout(() => { notification_queue[0].show() }, 1500)
  }
}

// '\b↓' Parser... Get it? Bytedown... \b↓? *sigh*
// To use a \b↓ token macro, use "\\<token>"
function parseBytedown(text) {
  const tokens = [
    { token: "i", replace: "<i>", close: "</i>" },
    { token: "f", replace: `<span style="opacity: 0.6">`, close: "</span>" }
  ]
  
  let output = text
  
  for (let i = 0; i < text.length; i++) {
    const char = text[i]
    const next_char = text[i + 1]
    
    if (char != "\\") continue
    
    for (let i = 0; i < tokens.length; i++) {
      const token = tokens[i].token
      if (next_char != tokens[i].token) continue
      
      const replace = tokens[i].replace
      const close = tokens[i].close
      
      output = output.replace("\\" + token, replace)
      if (close) output = output.replace("\\" + token, close)
    }
  }
  
  return output.replaceAll("\n", "<br>")
}
