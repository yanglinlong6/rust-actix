// @generated automatically by Diesel CLI.

diesel::table! {
    d_alarm_second_data (id) {
        id -> Bigint,
        sn -> Nullable<Longtext>,
        #[sql_name = "type"]
        type_ -> Nullable<Longtext>,
        name -> Nullable<Longtext>,
        alarm_time -> Nullable<Datetime>,
        alarm_address -> Nullable<Longtext>,
        leave_time -> Nullable<Datetime>,
        stay_time -> Nullable<Bigint>,
        lng -> Nullable<Longtext>,
        lat -> Nullable<Longtext>,
        event_time -> Nullable<Datetime>,
        del_flag -> Nullable<Tinyint>,
        unique_flag -> Nullable<Longtext>,
        create_time -> Nullable<Datetime>,
        update_time -> Nullable<Datetime>,
    }
}

diesel::table! {
    doctor_tb (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Nullable<Varchar>,
        age -> Nullable<Integer>,
        sex -> Nullable<Integer>,
        addtime -> Nullable<Datetime>,
    }
}

diesel::table! {
    hotrows (id) {
        id -> Integer,
        data -> Text,
        #[max_length = 45]
        data_type -> Varchar,
        #[max_length = 45]
        name -> Varchar,
        is_show -> Integer,
        created -> Datetime,
    }
}

diesel::table! {
    t_goods (id) {
        id -> Bigint,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        tel -> Varchar,
        #[max_length = 128]
        flag -> Varchar,
        user_id -> Bigint,
    }
}

diesel::table! {
    t_nes_device (sn) {
        #[max_length = 32]
        sn -> Varchar,
        #[max_length = 100]
        model_name -> Nullable<Varchar>,
    }
}

diesel::table! {
    t_order (id) {
        id -> Integer,
        #[max_length = 255]
        order_title -> Varchar,
        #[max_length = 255]
        order_content -> Varchar,
        status -> Integer,
        #[max_length = 255]
        remark -> Nullable<Varchar>,
    }
}

diesel::table! {
    t_user_demo (id) {
        id -> Integer,
        #[max_length = 15]
        name -> Varchar,
        #[max_length = 15]
        password -> Varchar,
        age -> Integer,
        #[max_length = 30]
        email -> Nullable<Varchar>,
        #[max_length = 11]
        tel -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        create_time -> Nullable<Datetime>,
        updata_time -> Nullable<Datetime>,
    }
}

diesel::table! {
    tb_user (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 50]
        password -> Varchar,
        #[max_length = 100]
        email -> Varchar,
    }
}

diesel::table! {
    user_login (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        gender -> Nullable<Integer>,
        age -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Nullable<Longtext>,
        email -> Nullable<Longtext>,
        age -> Nullable<Unsigned<Tinyint>>,
        birthday -> Nullable<Datetime>,
        member_number -> Nullable<Longtext>,
        activated_at -> Nullable<Datetime>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    d_alarm_second_data,
    doctor_tb,
    hotrows,
    t_goods,
    t_nes_device,
    t_order,
    t_user_demo,
    tb_user,
    user_login,
    users,
);
