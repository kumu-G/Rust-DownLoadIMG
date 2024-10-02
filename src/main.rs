use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let tmp_dir = Builder::new()
        .prefix("example")
        .tempdir_in(&PathBuf::from("./"))?;

    println!("tmp_dir: {tmp_dir:?}");
    // 将临时目录转换为PathBuf类型
    let tmp_dir = tmp_dir.into_path();

    let target = "https://picsum.photos/200/300";
    let response = reqwest::get(target).await?;
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: {fname}");
        // let fname = tmp_dir.path().join(fname);
        let fname = tmp_dir.join(fname);
        println!("will be located under: {fname:?}");
        File::create(fname)?
    };
    // let content = response.text().await?;
    // println!("content: {:?}", content.as_bytes().len());
    // copy(&mut content.as_bytes(), &mut dest)?;
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;
    // 在程序结束前手动删除临时目录
    // std::fs::remove_dir_all(tmp_dir)?;
    Ok(())
}

/*
这段代码的功能是从指定的URL下载文件并将其保存到临时目录中。

代码步骤如下：

1. 导入所需的库和模块。
2. 定义错误链。
3. 定义一个异步的main函数作为程序的入口。
4. 创建一个临时目录，并将其保存在tmp_dir变量中。
5. 将临时目录转换为PathBuf类型。
6. 指定要下载的文件的URL。
7. 使用reqwest库发送HTTP请求，并将响应保存在response变量中。
8. 创建一个文件用于保存下载的内容，文件名根据URL中的最后一部分生成。
9. 将文件名与临时目录的路径拼接，得到完整的文件路径。
10. 将文件创建为可写的文件对象，并保存在dest变量中。
11. 从响应中获取文件的内容，并将其复制到dest文件中。
12. 返回结果。

该代码使用了一些第三方库，例如error_chain、reqwest和tempfile。它通过异步编程的方式实现了文件的下载和保存。 */
