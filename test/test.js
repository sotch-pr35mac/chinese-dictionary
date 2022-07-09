const assert = require('assert');
const dictionary = require('../lib/index.js');

dictionary.init().then(() => {
	describe('Chinese Dictionary', () => {
		describe('query', () => {
			it('should return the correct dictionary entries', () => {
				return dictionary.query('test').then(result => {
					assert.equal(result[0].traditional, '實驗');
				});
			});
		});
		describe('queryByChinese', () => {
			it('should return the correct dictionary entries', () => {
				return dictionary.queryByChinese('天气').then(result => {
					assert.equal(result[0].english[0], 'weather');
				});
			});
		});
		describe('queryByPinyin', () => {
			it('should return the correct dictionary entries', () => {
				return dictionary.queryByPinyin('tianqi').then(result => {
					assert.equal(result[0].english[0], 'weather');
				});
			});
		});
		describe('queryByEnglish', () => {
			it('should return the correct dictionary entries', () => {
				return dictionary.queryByEnglish('test').then(result => {
					assert.equal(result[0].simplified, '实验');
				});
			});
		});
		describe('segment', () => {
			it('should correctly segment a sentence', () => {
				return dictionary.segment('今天的天气还可以吧').then(result => {
					assert.equal(result[0], '今天');
				});
			});
		});
		describe('isSimplified', () => {
			it('should return true for simplified string', () => {
				assert.equal(dictionary.isSimplified('简体字'), true);
			});
			it('should return false for traditional string', () => {
				assert.equal(dictionary.isSimplified('簡體字'), false);
			});
		});
		describe('isTraditional', () => {
			it('should return true for traditional string', () => {
				assert.equal(dictionary.isTraditional('繁體字'), true);
			});
			it('should return false for simplified string', () => {
				assert.equal(dictionary.isTraditional('繁体字'), false);
			});
		});
		describe('convertToTraditional', () => {
			it('should return traditional characters when given a simplified string', () => {
				return dictionary.convertToTraditional('汉语').then(result => {
					assert.equal(dictionary.isTraditional(result), true);
				});
			});
		});
		describe('convertToSimplified', () => {
			it('should return simplified characters when given a traditional string', () => {
				return dictionary.convertToSimplified('漢語').then(result => {
					assert.equal(dictionary.isSimplified(result), true);
				});
			});
		});
		describe('classify', () => {
			it('should correctly classify an english string', () => {
				return dictionary.classify('test').then(result => {
					assert.equal(result, 'EN');
				});
			});
			it('should correctly classify a pinyin string', () => {
				return dictionary.classify('shiyan').then(result => {
					assert.equal(result, 'PY');
				});
			});
			it('should correctly classify a string of Chinese characters', () => {
				return dictionary.classify('实验').then(result => {
					assert.equal(result, 'ZH');
				});
			});
		});
	});
});
