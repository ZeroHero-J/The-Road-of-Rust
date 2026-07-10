use polars::prelude::*;

fn main() -> PolarsResult<()> {
    // 1. 读入数据（假设你有一个零件测量的 CSV 或者是 Excel 转来的数据）
    let df = df!(
        "零件编号" => &["A01", "A02", "A03", "A04"],
        "实测误差" => &[0.02, 0.06, 0.01, 0.08]
    )?;

    // 2. 施展降维打击：一行代码筛选出误差大于 0.05 的所有不合格零件
    let mask = df.column("实测误差")?.gt(0.05)?;
    let filtered_df = df.filter(&mask)?;

    // 3. 打印结果
    println!("--- 抓到以下不合格零件 ---");
    println!("{}", filtered_df);

    Ok(())
}
