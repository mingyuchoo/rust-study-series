# actix_sqlx_mysql

## How to create project

```sh
cargo new <project-name>
cd <project-name>
```

```toml
# Cargo.toml

[dependencies]
actix-web = "4"
sqlx = { version = "0.6", features = [ "runtime-actix-native-tls" , "mysql" ] }
```

## MySQL

```sql
CREATE TABLE `tb_sequence` (
  `seq_name` varchar(4) NOT NULL,
  `seq_no` int NOT NULL,
  PRIMARY KEY (`seq_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COMMENT='tb_sequence';

CREATE TABLE `members` (
  `id` varchar(9) CHARACTER SET utf8 COLLATE utf8_general_ci NOT NULL,
  `name` varchar(64) CHARACTER SET utf8 COLLATE utf8_general_ci DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8 COLLATE=utf8_bin;


CREATE DEFINER=`postgres`@`%` FUNCTION `postgres`.`fn_get_seq_8`(p_seq_name VARCHAR(4)) RETURNS varchar(8) CHARSET utf8
begin
    DECLARE RTN_VAL VARCHAR(8);

    INSERT INTO tb_sequence (seq_name, seq_no)
         values (p_seq_name, LAST_INSERT_ID(1))
    ON DUPLICATE KEY UPDATE seq_no=LAST_INSERT_ID(seq_no+1);

    set @ret = row_count();

    if @ret = 0 then
        set RTN_VAL = '0';
    else
        SET RTN_VAL = (SELECT CONCAT(p_seq_name,  LPAD(LAST_INSERT_ID(),4,'0')));
    end if;

       RETURN RTN_VAL;
END
```

## How to run

connect to `http://localhost:9090/`

## References

- <https://velog.io/@inyong_pang/MySQL-MySQL-Sequence-%EB%A7%8C%EB%93%A4%EA%B8%B0>
- <https://github.com/launchbadge/sqlx>
- <https://actix.rs/>
