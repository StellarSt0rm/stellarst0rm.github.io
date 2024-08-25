function main(apps_data) {
  console.log(apps_data)
  
  for (let i = 0; i < apps_data.apps.length; i++) {
    const desktop_window = new DesktopWindow(apps_data.apps[i])
    
    if (apps_data.apps[i].name != "Portfolio") desktop_window.hide()
  }
  
  /*setTimeout(() => {
    function random_notification() {
      const notifications = apps_data.notifications
      const random = Math.floor(Math.random() * notifications.length)
      
      if (!notification_queue[0]) new DesktopNotification(notifications[random])
      
      setTimeout(random_notification, 30000 * (Math.floor(Math.random() * 8) + 1) + 2000)
    }
    
    random_notification()
  }, 40000 * (Math.floor(Math.random() * 4) + 1))*/
}
