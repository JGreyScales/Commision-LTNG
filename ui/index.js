const { invoke } = window.__TAURI__.tauri 
const delay = ms => new Promise(res => setTimeout(res, ms));


let percents = document.getElementsByClassName("percents")[0].children

async function queryAverage(i){
    await invoke ('calculateAverage', {indexcount:i}).then( returnValue => {
        percents[i - 1].innerHTML =  returnValue[0] + "%<br>" + returnValue[1]
    })
}

async function asyncCall(i){
    await delay(1);
    await queryAverage(i)
}


window.addEventListener("DOMContentLoaded", () => {
    for (let i = 0; i != 8; i++){
        percents[i].innerHTML = "Loading..."
        asyncCall(3)
        // asyncCall(i)
        break
    }
})


