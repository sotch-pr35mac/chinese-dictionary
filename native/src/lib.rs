extern crate chinese_dictionary;

use chinese_dictionary::{
    classify, init, is_simplified, is_traditional, query, query_by_chinese, query_by_english,
    query_by_pinyin, simplified_to_traditional, tokenize, traditional_to_simplified,
    ClassificationResult::*, WordEntry,
};
use neon::prelude::*;
use std::sync::Once;

static INIT: Once = Once::new();

#[macro_export]
macro_rules! convert_measure_word {
    ($context:expr, $measure_word:expr) => {{
        let measure_word_js = JsObject::new(&mut $context);

        let traditional = $context.string(&$measure_word.traditional);
        let simplified = $context.string(&$measure_word.simplified);
        let pinyin_marks = $context.string(&$measure_word.pinyin_marks);
        let pinyin_numbers = $context.string(&$measure_word.pinyin_numbers);

        measure_word_js
            .set(&mut $context, "traditional", traditional)
            .unwrap();
        measure_word_js
            .set(&mut $context, "simplified", simplified)
            .unwrap();
        measure_word_js
            .set(&mut $context, "pinyinMarks", pinyin_marks)
            .unwrap();
        measure_word_js
            .set(&mut $context, "pinyinNumbers", pinyin_numbers)
            .unwrap();

        measure_word_js
    }};
}

#[macro_export]
macro_rules! convert_word_entry {
    ($context:expr, $word_entry:expr) => {{
        let word = JsObject::new(&mut $context);

        let traditional = $context.string(&$word_entry.traditional);
        let simplified = $context.string(&$word_entry.simplified);
        let pinyin_marks = $context.string(&$word_entry.pinyin_marks);
        let pinyin_numbers = $context.string(&$word_entry.pinyin_numbers);
        let hash = $context.string(format!("{}", $word_entry.hash));
        let hsk = $context.number($word_entry.hsk as u32);
        let word_id = $context.number($word_entry.word_id);

        let english = JsArray::new(&mut $context, $word_entry.english.len() as u32);
        for (i, definition) in $word_entry.english.iter().enumerate() {
            let definition_js = $context.string(definition);
            english.set(&mut $context, i as u32, definition_js).unwrap();
        }

        let measure_words = JsArray::new(&mut $context, $word_entry.measure_words.len() as u32);
        for (i, measure_word) in $word_entry.measure_words.iter().enumerate() {
            let measure_word_js = convert_measure_word!($context, measure_word);
            measure_words
                .set(&mut $context, i as u32, measure_word_js)
                .unwrap();
        }

        let tone_marks = JsArray::new(&mut $context, $word_entry.tone_marks.len() as u32);
        for (i, tone_mark) in $word_entry.tone_marks.iter().enumerate() {
            let tone_mark_js = $context.number(*tone_mark as u32);
            tone_marks
                .set(&mut $context, i as u32, tone_mark_js)
                .unwrap();
        }

        word.set(&mut $context, "traditional", traditional).unwrap();
        word.set(&mut $context, "simplified", simplified).unwrap();
        word.set(&mut $context, "pinyinMarks", pinyin_marks)
            .unwrap();
        word.set(&mut $context, "pinyinNumbers", pinyin_numbers)
            .unwrap();
        word.set(&mut $context, "hash", hash).unwrap();
        word.set(&mut $context, "hsk", hsk).unwrap();
        word.set(&mut $context, "wordId", word_id).unwrap();
        word.set(&mut $context, "english", english).unwrap();
        word.set(&mut $context, "measureWords", measure_words)
            .unwrap();
        word.set(&mut $context, "toneMarks", tone_marks).unwrap();

        word
    }};
}

fn neon_init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    INIT.call_once(|| {
        init();
    });
    Ok(cx.undefined())
}

fn neon_classify(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(match classify(&input) {
        EN => "EN",
        PY => "PY",
        ZH => "ZH",
        _ => "UN",
    }))
}

fn neon_convert_to_simplified(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(traditional_to_simplified(&input)))
}

fn neon_convert_to_traditional(mut cx: FunctionContext) -> JsResult<JsString> {
    let input = cx.argument::<JsString>(0)?.value();
    Ok(cx.string(simplified_to_traditional(&input)))
}

fn neon_is_traditional(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let input = cx.argument::<JsString>(0)?.value();
    Ok(cx.boolean(is_traditional(&input)))
}

fn neon_is_simplified(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let input = cx.argument::<JsString>(0)?.value();
    Ok(cx.boolean(is_simplified(&input)))
}

fn neon_segment(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input = cx.argument::<JsString>(0)?.value();
    let tokens: Vec<&str> = tokenize(&input);
    let js_array = JsArray::new(&mut cx, tokens.len() as u32);
    for (i, obj) in tokens.iter().enumerate() {
        let js_string = cx.string(obj);
        js_array.set(&mut cx, i as u32, js_string).unwrap();
    }
    Ok(js_array)
}

fn neon_query_by_english(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input = cx.argument::<JsString>(0)?.value().to_lowercase();
    let words: Vec<&WordEntry> = query_by_english(&input);
    let js_array = JsArray::new(&mut cx, words.len() as u32);
    for (i, word_entry) in words.iter().enumerate() {
        let word = convert_word_entry!(cx, word_entry);
        js_array.set(&mut cx, i as u32, word).unwrap();
    }
    Ok(js_array)
}

fn neon_query_by_pinyin(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input = cx.argument::<JsString>(0)?.value().to_lowercase();
    let words: Vec<&WordEntry> = query_by_pinyin(&input);
    let js_array = JsArray::new(&mut cx, words.len() as u32);
    for (i, word_entry) in words.iter().enumerate() {
        let word = convert_word_entry!(cx, word_entry);
        js_array.set(&mut cx, i as u32, word).unwrap();
    }
    Ok(js_array)
}

fn neon_query_by_chinese(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input = cx.argument::<JsString>(0)?.value().to_lowercase();
    let words: Vec<&WordEntry> = query_by_chinese(&input);
    let js_array = JsArray::new(&mut cx, words.len() as u32);
    for (i, word_entry) in words.iter().enumerate() {
        let word = convert_word_entry!(cx, word_entry);
        js_array.set(&mut cx, i as u32, word).unwrap();
    }
    Ok(js_array)
}

fn neon_query(mut cx: FunctionContext) -> JsResult<JsArray> {
    let input = cx.argument::<JsString>(0)?.value().to_lowercase();
    let words: Vec<&WordEntry> = match query(&input) {
        None => vec![],
        Some(result) => result,
    };
    let js_array = JsArray::new(&mut cx, words.len() as u32);
    for (i, word_entry) in words.iter().enumerate() {
        let word = convert_word_entry!(cx, word_entry);
        js_array.set(&mut cx, i as u32, word).unwrap();
    }
    Ok(js_array)
}

register_module!(mut cx, {
    cx.export_function("init", neon_init)?;
    cx.export_function("classify", neon_classify)?;
    cx.export_function("convertToSimplified", neon_convert_to_simplified)?;
    cx.export_function("convertToTraditional", neon_convert_to_traditional)?;
    cx.export_function("isTraditional", neon_is_traditional)?;
    cx.export_function("isSimplified", neon_is_simplified)?;
    cx.export_function("segment", neon_segment)?;
    cx.export_function("queryByEnglish", neon_query_by_english)?;
    cx.export_function("queryByPinyin", neon_query_by_pinyin)?;
    cx.export_function("queryByChinese", neon_query_by_chinese)?;
    cx.export_function("query", neon_query)?;
    Ok(())
});
