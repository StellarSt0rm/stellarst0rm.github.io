globalThis.notification_queue = []

class DesktopNotification {
  constructor(data) {
    if (!data) throw Error("Argument 'data' must be defined!")
    
    if (!data.app) throw Error("Argument 'data' must have 'app' attribute!")
    if (!data.text) throw Error("Argument 'data' must have 'text' attribute!")
    if (!data.image) throw Error("Argument 'data' must have 'image' attribute!")
    
    
  }
  
  delete() {
    return
  }
}
