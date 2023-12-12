from selenium import webdriver
from selenium.webdriver.chrome.service import Service

# 指定 chromedriver 的路径
service = Service('/Users/Fox/Code/Rust/rust-react/python/chromedriver')
driver = webdriver.Chrome(service=service)
driver.get('http://www.baidu.com')
driver.implicitly_wait(10)
driver.find_element('xpath', '//*[@id="kw"]').send_keys('日期')
driver.find_element('id', 'su').click()
driver.quit()