const { invoke } = window.__TAURI__.tauri;

////////////////////////////////////////////////////////

// silly little function that tricks the browser into freezing, even in an async environ
const delay = ms => new Promise(res => setTimeout(res, ms));

let percents = document.getElementsByClassName("percents")[0].children;
let submitHolders = document.getElementsByClassName("number");

async function queryAverage(i){
    return await invoke ('calculate_average', {indexcount:i});
}

// silly little function that can be called from the main thread
async function asyncCall(i){
    await delay(1);
    return await queryAverage(i);
}

window.addEventListener("DOMContentLoaded", () => {
    for (let i = 1; i != 9; i++){
        percents[i - 1].innerHTML = "Loading...";
        asyncCall(i).then( returnValue => {
            percents[i - 1].innerHTML =  returnValue[0] + "%<br>" + returnValue[1]
        });
    };
})
////////////////////////////////////////////////////////

function updateJSON(){
    let key = ""
    
    for (let i = 0; i != 8; i++){
        let value = submitHolders[i].value
        if (value.toString().length === 0 || value.toString().length > 2){
            console.log("Stopped")
            return
        }
        key += value + " "
        submitHolders[i].value = ""
    }
    invoke ('add_new_key', {new_key:key});
}

////////////////////////////////////////////////////////

function exit(){
    invoke ('exit_process', {})
}
////////////////////////////////////////////////////////