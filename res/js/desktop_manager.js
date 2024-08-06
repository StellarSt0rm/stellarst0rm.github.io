function main(apps_data) {
  const desktop = document.querySelector("desktop")

  console.log(apps_data)
  
  desktop.style.opacity = "1"
  desktop.style.animation = "zoom-in 1s"
  
  windowa = new DesktopWindow("meow", "aa")
  windowaa = new DesktopWindow("meow3", "aaa")
}
