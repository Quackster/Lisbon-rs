//! Mirrors `net.h4bbo.lisbon.util.config.writer.GameConfigWriter`.

use std::collections::HashMap;
use std::io::Write;

use super::ConfigWriter;

pub struct GameConfigWriter;

impl GameConfigWriter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GameConfigWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigWriter for GameConfigWriter {
    fn set_configuration_defaults(&self) -> HashMap<String, String> {
        let mut c = HashMap::new();
        macro_rules! p {
            ($k:expr, $v:expr) => {
                c.insert($k.to_string(), $v.to_string());
            };
        }

        p!("fuck.aaron", "true");
        p!("max.connections.per.ip", "2");
        p!("normalise.input.strings", "false");

        p!("room.dispose.timer.enabled", "true");
        p!("room.dispose.timer.seconds", "60");

        p!("welcome.message.enabled", "false");
        p!("welcome.message.content", "Hello, %username%! And welcome to the Kepler server!");

        p!("roller.tick.default", "2000");

        p!("afk.timer.seconds", "900");
        p!("sleep.timer.seconds", "300");
        p!("carry.timer.seconds", "300");

        p!("stack.height.limit", "8");
        p!("roomdimmer.scripting.allowed", "false");

        p!("credits.scheduler.enabled", "true");
        p!("credits.scheduler.timeunit", "MINUTES");
        p!("credits.scheduler.interval", "15");
        p!("credits.scheduler.amount", "20");

        p!("chat.garbled.text", "true");
        p!("chat.bubble.timeout.seconds", "15");

        p!("messenger.max.friends.nonclub", "100");
        p!("messenger.max.friends.club", "600");

        p!("battleball.create.game.enabled", "true");
        p!("battleball.start.minimum.active.teams", "2");
        p!("battleball.preparing.game.seconds", "10");
        p!("battleball.game.lifetime.seconds", "180");
        p!("battleball.restart.game.seconds", "30");
        p!("battleball.ticket.charge", "2");
        p!("battleball.increase.points", "true");

        p!("game.finished.listing.expiry.seconds", "300");

        p!("snowstorm.create.game.enabled", "true");
        p!("snowstorm.start.minimum.active.teams", "2");
        p!("snowstorm.preparing.game.seconds", "10");
        // p!("snowstorm.game.lifetime.seconds", "180"); // commented in Java
        p!("snowstorm.restart.game.seconds", "30");
        p!("snowstorm.ticket.charge", "2");
        p!("snowstorm.increase.points", "true");

        p!("poker.entry.price", "0");
        p!("poker.entry.price.only.in.rooms", "");
        p!("poker.entry.price.redistribute", "true");
        p!("poker.entry.price.redistribute.on.tie", "true");
        p!("poker.entry.price.redistribute.only.in.rooms", "");

        p!("poker.reward.min.player", "2");
        p!("poker.reward.min.player.only.in.rooms", "");
        p!("poker.reward.credits.bonus", "0");
        p!("poker.reward.credits.bonus.on.tie", "false");
        p!("poker.reward.credits.bonus.only.in.rooms", "");

        p!("poker.reward.rares", "");
        p!("poker.reward.rares.only.in.rooms", "");
        p!("poker.reward.rares.quantity", "1");
        p!("poker.reward.rares.on.tie", "false");

        p!("poker.reward.tickets", "0");
        p!("poker.reward.tickets.on.tie", "false");
        p!("poker.reward.tickets.only.in.rooms", "");

        p!("poker.announce.winner", "false");
        p!("poker.announce.winner.only.in.rooms", "0");
        p!("poker.announce.rewards", "false");
        p!("poker.announce.rewards.only.in.rooms", "");

        p!("tutorial.enabled", "true");
        p!("profile.editing", "true");
        p!("vouchers.enabled", "true");

        p!("shutdown.minutes", "1");

        p!("reset.sso.after.login", "true");
        p!("room.bots.enabled", "true");

        p!("navigator.show.hidden.rooms", "false");

        p!("rare.cycle.page.text", "Okay this thing is fucking epic!<br><br>The time until the next rare is {rareCountdown}!");
        p!("rare.cycle.tick.time", "0");
        p!("rare.cycle.page.id", "2");
        p!("rare.cycle.refresh.timeunit", "DAYS");
        p!("rare.cycle.refresh.interval", "1");

        p!("rare.cycle.reuse.timeunit", "DAYS");
        p!("rare.cycle.reuse.interval", "7");

        p!("rare.cycle.reuse.throne.timeunit", "DAYS");
        p!("rare.cycle.reuse.throne.interval", "30");

        p!("club.gift.timeunit", "DAYS");
        p!("club.gift.interval", "31");
        p!("club.gift.present.label", "You have just received your monthly club gift!");

        p!("users.figure.parts.default", "100,105,110,115,120,125,130,135,140,145,150,155,160,165,170,175,176,177,178,180,185,190,195,200,205,206,207,210,215,220,225,230,235,240,245,250,255,260,265,266,267,270,275,280,281,285,290,295,300,305,500,505,510,515,520,525,530,535,540,545,550,555,565,570,575,580,585,590,595,596,600,605,610,615,620,625,626,627,630,635,640,645,650,655,660,665,667,669,670,675,680,685,690,695,696,700,705,710,715,720,725,730,735,740");
        p!("users.figure.parts.club", "100,105,110,115,120,125,130,135,140,145,150,155,160,165,170,175,176,177,178,180,185,190,195,200,205,206,207,210,215,220,225,230,235,240,245,250,255,260,265,266,267,270,275,280,281,285,290,295,300,305,500,505,510,515,520,525,530,535,540,545,550,555,565,570,575,580,585,590,595,596,600,605,610,615,620,625,626,627,630,635,640,645,650,655,660,665,667,669,670,675,680,685,690,695,696,700,705,710,715,720,725,730,735,740,800,801,802,803,804,805,806,807,808,809,810,811,812,813,814,815,816,817,818,819,820,821,822,823,824,825,826,827,828,829,830,831,832,833,834,835,836,837,838,839,840,841,842,843,844,845,846,847,848,849,850,851,852,853,854,855,856,857,858,859,860,861,862,863,864,865,866,867,868,869,870,871,872,873");

        p!("events.category.count", "11");
        p!("events.expiry.minutes", "120");

        p!("disable.purchase.successful.alert", "false");

        p!("recycler.max.time.to.collect.seconds", "1800");
        p!("recycler.session.length.seconds", "3600");
        p!("recycler.item.quarantine.seconds", "2592000");

        p!("happy.hour.weekday.start", "17:00:00");
        p!("happy.hour.weekday.end", "18:00:00");

        p!("happy.hour.weekend.start", "12:00:00");
        p!("happy.hour.weekend.end", "13:00:00");

        p!("players.online", "0");

        p!("rare.cycle.pages", "28,3|29,3|31,3|32,3|33,3|34,3|35,3|36,3|40,3|43,3|30,6|37,6|38,6|39,6|44,6");

        p!("reward.credits.winner.range", "10-20");
        p!("reward.credits.loser.range", "0-4");

        p!("guides.group.id", "1");

        c
    }

    fn set_configuration_data(
        &self,
        _config: &HashMap<String, String>,
        _writer: &mut dyn Write,
    ) -> std::io::Result<()> {
        // Java implementation is empty.
        Ok(())
    }
}
