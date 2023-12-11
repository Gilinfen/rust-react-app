from selenium import webdriver

driver = webdriver.Chrome()
driver.get('http://www.baidu.com')
driver.implicitly_wait(10)
driver.find_element('xpath', '//*[@id="kw"]').send_keys('日期')
driver.find_element('id', 'su').click()
driver.quit()