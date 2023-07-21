const { invoke } = window.__TAURI__.tauri 


let percents = document.getElementsByClassName("percents")[0].children

async function queryAverage(i){
    await invoke ('calculateAverage', {indexcount:i}).then( returnValue => {
        percents[i].innerHTML =  returnValue[0] + "%<br>" + returnValue[1]
    })
}

async function asyncCall(i){
    await queryAverage(i)
}

window.addEventListener("DOMContentLoaded", () => {
    for (let i = 0; i != 8; i++){
        percents[i].innerHTML = "Loading..."
        asyncCall(i)
    }
})


