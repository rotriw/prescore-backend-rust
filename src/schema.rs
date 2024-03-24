pub mod prescore {
    diesel::table! {
        /// Representation of the `prescore.exam` table.
        ///
        /// (Automatically generated by Diesel.)
        prescore.exam (id) {
            /// The `id` column of the `prescore.exam` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            id -> Int8,
            /// The `user_id` column of the `prescore.exam` table.
            ///
            /// Its SQL type is `Text`.
            ///
            user_id -> Text,
            /// The `exam_id` column of the `prescore.exam` table.
            ///
            /// Its SQL type is `Text`.
            ///
            exam_id -> Text,
            /// The `paper_id` column of the `prescore.exam` table.
            ///
            /// Its SQL type is `Text`.
            ///
            paper_id -> Text,
            /// The `subject_name` column of the `prescore.exam` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            subject_name -> Nullable<Text>,
            /// The `subject_id` column of the `prescore.exam` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            subject_id -> Nullable<Text>,
            /// The `standard_score` column of the `prescore.exam` table.
            ///
            /// Its SQL type is `Nullable<Float8>`.
            ///
            standard_score -> Nullable<Float8>,
            /// The `user_score` column of the `prescore.exam` table.
            ///
            /// Its SQL type is `Nullable<Float8>`.
            ///
            user_score -> Nullable<Float8>,
            /// The `diagnostic_score` column of the `prescore.exam` table.
            ///
            /// Its SQL type is `Nullable<Float8>`.
            ///
            diagnostic_score -> Nullable<Float8>,
        }
    }

    diesel::table! {
        /// Representation of the `prescore.user` table.
        ///
        /// (Automatically generated by Diesel.)
        prescore.user (id) {
            /// The `id` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Int8`.
            ///
            id -> Int8,
            /// The `user_id` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            user_id -> Nullable<Text>,
            /// The `name` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            name -> Nullable<Text>,
            /// The `login_name` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            login_name -> Nullable<Text>,
            /// The `hashed_token` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            hashed_token -> Nullable<Text>,
            /// The `school_id` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            school_id -> Nullable<Text>,
            /// The `school_name` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            school_name -> Nullable<Text>,
            /// The `division_id` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            division_id -> Nullable<Text>,
            /// The `division_name` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            division_name -> Nullable<Text>,
            /// The `class_id` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            class_id -> Nullable<Text>,
            /// The `class_name` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            class_name -> Nullable<Text>,
            /// The `child_id` column of the `prescore.user` table.
            ///
            /// Its SQL type is `Nullable<Text>`.
            ///
            child_id -> Nullable<Text>,
        }
    }

    diesel::allow_tables_to_appear_in_same_query!(
        exam,
        user,
    );
}
