const { invoke } = window.__TAURI__.tauri 
const delay = ms => new Promise(res => setTimeout(res, ms));


let percents = document.getElementsByClassName("percents")[0].children

async function queryAverage(i){
    return await invoke ('calculateAverage', {indexcount:i})
}

async function asyncCall(i){
    await delay(1);
    return await queryAverage(i)
}


window.addEventListener("DOMContentLoaded", () => {
    for (let i = 1; i != 9; i++){
        percents[i - 1].innerHTML = "Loading..."
        console.log(percents[7].innerHTML)
        asyncCall(i).then( returnValue => {
            percents[i - 1].innerHTML =  returnValue[0] + "%<br>" + returnValue[1]
        })
    }
})


