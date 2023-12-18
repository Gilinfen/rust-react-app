from selenium import webdriver
from selenium.webdriver.chrome.service import Service
import json

# 打开并读取JSON文件
with open('./settings.json', 'r') as file:
    data = json.load(file)

# 打印读取的数据
print(f"chromedriver：{data['chromedriver']}")

# 指定 chromedriver 的路径
service = Service(data["chromedriver"])
driver = webdriver.Chrome(service=service)
driver.get('http://www.baidu.com')
driver.implicitly_wait(10)
driver.find_element('xpath', '//*[@id="kw"]').send_keys('日期')
driver.find_element('id', 'su').click()
driver.quit()