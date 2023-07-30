from selenium import webdriver
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.common.by import By
import json, time

def generateKeys(elements, bonuses):
    global collectedKeys
    key = ""
    for (index, number)in enumerate(elements):
        if index % 8 == 0:
            if key != "":
                collectedKeys["Winners"].append(f"{bonuses[round(index/8)].text[6::]} {key}")
            key = ""
        else:
            key += f"{number.text} "

def getElements(browser):
    return browser.find_elements(By.CLASS_NAME, "pastWinNumber")

collectedKeys = {"Winners":[]}

driverPath = "C:/Users/gameb/Downloads/chromedriver_win32/chromedriver.exe"
bravePath = "C:/Program Files/BraveSoftware/Brave-Browser/Application/brave.exe"

service=Service(driverPath)
option = webdriver.ChromeOptions()
option.binary_location = bravePath
browser = webdriver.Chrome(service=service, options=option)

browser.get("https://www.wclc.com/winning-numbers/lotto-max-extra.htm")

for (index, page) in enumerate(browser.find_elements(By.CLASS_NAME, "pastMonthYearWinners")[2::]):
    bonuses = browser.find_elements(By.CLASS_NAME, "pastWinNumberBonus")
    print(f"Collecting page {index + 1}")
    generateKeys(getElements(browser), bonuses)
    print("Loading next page")
    page.click()
    time.sleep(1)

outfile = open("C:/Programming/Commision-LTNG/ui/Assets/Winners.json", "+w")
json.dump(collectedKeys, outfile, indent=4)
outfile.close()