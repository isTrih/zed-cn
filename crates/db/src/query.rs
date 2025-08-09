#[macro_export]
macro_rules! query {
    ($vis:vis fn $id:ident() -> Result<()> { $($sql:tt)+ }) => {
        $vis fn $id(&self) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.exec(sql_stmt)?().context(::std::format!(
                "在 {} 中出错，exec 执行或解析失败：{}",
                ::std::stringify!($id),
                sql_stmt,
            ))
        }
    };
    ($vis:vis async fn $id:ident() -> Result<()> { $($sql:tt)+ }) => {
        $vis async fn $id(&self) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.exec(sql_stmt)?().context(::std::format!(
                    "在 {} 中出错，exec 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
            }).await
        }
    };
    ($vis:vis fn $id:ident($($arg:ident: $arg_type:ty),+) -> Result<()> { $($sql:tt)+ }) => {
        $vis fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.exec_bound::<($($arg_type),+)>(sql_stmt)?(($($arg),+))
                .context(::std::format!(
                    "在 {} 中出错，exec_bound 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
        }
    };
    ($vis:vis async fn $id:ident($arg:ident: $arg_type:ty) -> Result<()> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $arg: $arg_type) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            self.write(move |connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.exec_bound::<$arg_type>(sql_stmt)?($arg)
                    .context(::std::format!(
                        "在 {} 中出错，exec_bound 执行或解析失败：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis async fn $id:ident($($arg:ident: $arg_type:ty),+) -> Result<()> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<()> {
            use $crate::anyhow::Context;

            self.write(move |connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.exec_bound::<($($arg_type),+)>(sql_stmt)?(($($arg),+))
                    .context(::std::format!(
                        "在 {} 中出错，exec_bound 执行或解析失败：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident() ->  Result<Vec<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis fn $id(&self) -> $crate::anyhow::Result<Vec<$return_type>> {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select::<$return_type>(sql_stmt)?()
                .context(::std::format!(
                    "在 {} 中出错，select_row 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
        }
    };
    ($vis:vis async fn $id:ident() -> Result<Vec<$return_type:ty>> { $($sql:tt)+ }) => {
        pub async fn $id(&self) -> $crate::anyhow::Result<Vec<$return_type>> {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select::<$return_type>(sql_stmt)?()
                    .context(::std::format!(
                        "在 {} 中出错，select_row 执行或解析失败：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident($($arg:ident: $arg_type:ty),+) -> Result<Vec<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<Vec<$return_type>> {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                .context(::std::format!(
                    "在 {} 中出错，exec_bound 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
        }
    };
    ($vis:vis async fn $id:ident($($arg:ident: $arg_type:ty),+) -> Result<Vec<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<Vec<$return_type>> {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                    .context(::std::format!(
                        "在 {} 中出错，exec_bound 执行或解析失败：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident() ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis fn $id(&self) -> $crate::anyhow::Result<Option<$return_type>> {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select_row::<$return_type>(sql_stmt)?()
                .context(::std::format!(
                    "在 {} 中出错，select_row 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
        }
    };
    ($vis:vis async fn $id:ident() ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis async fn $id(&self) -> $crate::anyhow::Result<Option<$return_type>> {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_row::<$return_type>(sql_stmt)?()
                    .context(::std::format!(
                        "在 {} 中出错，select_row 执行或解析失败：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident($arg:ident: $arg_type:ty) ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis fn $id(&self, $arg: $arg_type) -> $crate::anyhow::Result<Option<$return_type>>  {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select_row_bound::<$arg_type, $return_type>(sql_stmt)?($arg)
                .context(::std::format!(
                    "在 {} 中出错，select_row_bound 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))

        }
    };
    ($vis:vis fn $id:ident($($arg:ident: $arg_type:ty),+) ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<Option<$return_type>>  {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select_row_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                .context(::std::format!(
                    "在 {} 中出错，select_row_bound 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))

        }
    };
    ($vis:vis async fn $id:ident($($arg:ident: $arg_type:ty),+) ->  Result<Option<$return_type:ty>> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $($arg: $arg_type),+) -> $crate::anyhow::Result<Option<$return_type>>  {
            use $crate::anyhow::Context;


            self.write(move |connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_row_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                    .context(::std::format!(
                        "在 {} 中出错，select_row_bound 执行或解析失败：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident() ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
        $vis fn $id(&self) ->  $crate::anyhow::Result<$return_type>  {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select_row::<$return_type>(sql_stmt)?()
                .context(::std::format!(
                    "在 {} 中出错，select_row_bound 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))?
                .context(::std::format!(
                    "在 {} 中出错，select_row_bound 期望单行结果但未找到：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
        }
    };
    ($vis:vis async fn $id:ident() ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
        $vis async fn $id(&self) ->  $crate::anyhow::Result<$return_type>  {
            use $crate::anyhow::Context;

            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_row::<$return_type>(sql_stmt)?()
                    .context(::std::format!(
                        "在 {} 中出错，select_row_bound 执行或解析失败：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))?
                    .context(::std::format!(
                        "在 {} 中出错，select_row_bound 期望单行结果但未找到：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
    ($vis:vis fn $id:ident($arg:ident: $arg_type:ty) ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
        pub fn $id(&self, $arg: $arg_type) ->  $crate::anyhow::Result<$return_type>  {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select_row_bound::<$arg_type, $return_type>(sql_stmt)?($arg)
                .context(::std::format!(
                    "在 {} 中出错，select_row_bound 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))?
                .context(::std::format!(
                    "在 {} 中出错，select_row_bound 期望单行结果但未找到：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
        }
    };
    ($vis:vis fn $id:ident($($arg:ident: $arg_type:ty),+) ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
        $vis fn $id(&self, $($arg: $arg_type),+) ->  $crate::anyhow::Result<$return_type>  {
            use $crate::anyhow::Context;

            let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

            self.select_row_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                .context(::std::format!(
                    "在 {} 中出错，select_row_bound 执行或解析失败：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))?
                .context(::std::format!(
                    "在 {} 中出错，select_row_bound 期望单行结果但未找到：{}",
                    ::std::stringify!($id),
                    sql_stmt
                ))
        }
    };
    ($vis:vis fn async $id:ident($($arg:ident: $arg_type:ty),+) ->  Result<$return_type:ty> { $($sql:tt)+ }) => {
        $vis async fn $id(&self, $($arg: $arg_type),+) ->  $crate::anyhow::Result<$return_type>  {
            use $crate::anyhow::Context;


            self.write(|connection| {
                let sql_stmt = $crate::sqlez_macros::sql!($($sql)+);

                connection.select_row_bound::<($($arg_type),+), $return_type>(sql_stmt)?(($($arg),+))
                    .context(::std::format!(
                        "在 {} 中出错，select_row_bound 执行或解析失败：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))?
                    .context(::std::format!(
                        "在 {} 中出错，select_row_bound 期望单行结果但未找到：{}",
                        ::std::stringify!($id),
                        sql_stmt
                    ))
            }).await
        }
    };
}
