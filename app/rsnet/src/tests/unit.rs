#[cfg(test)]
mod calculation {
    use crate::calculations::*;
    #[test]
    fn test_normalize_number() {
        let mut hosts: Vec<u32> = vec![2, 5, 7, 120, 253, 254, 1020, 63];
        for index in 0..hosts.len() {
            normalize_number(&mut hosts[index]);
        }
        assert_eq!(4, hosts[0]);
        assert_eq!(8, hosts[1]);
        assert_eq!(16, hosts[2]);
        assert_eq!(128, hosts[3]);
        assert_eq!(256, hosts[4]);
        assert_eq!(256, hosts[5]);
        assert_eq!(1024, hosts[6]);
        assert_eq!(128, hosts[7]);
    }

    #[test]
    fn test_broadcast() {
        let net_address: Address = Address::new(192, 168, 0, 0);
        let mask: Address = Address::new(255, 255, 255, 0);
        let wanted_broadcast: Address = Address::new(192, 168, 0, 255);
        let created_broadcast: Address = broadcast(&net_address, &mask);
        assert_eq!(
            wanted_broadcast.get().as_vec(),
            created_broadcast.get().as_vec()
        );

        let net_address: Address = Address::new(192, 168, 0, 0);
        let mask: Address = Address::new(255, 255, 255, 252);
        let wanted_broadcast: Address = Address::new(192, 168, 0, 3);
        let created_broadcast: Address = broadcast(&net_address, &mask);
        assert_eq!(
            wanted_broadcast.get().as_vec(),
            created_broadcast.get().as_vec()
        );
    }
    #[test]
    fn test_wildcard() {
        let mask: Address = Address::new(255, 255, 255, 0);
        let created_wildcard = wildcard(&mask);

        assert_eq!(created_wildcard.get().as_vec(), vec![0, 0, 0, 255]);

        let mask: Address = Address::new(255, 255, 252, 0);
        let created_wildcard = wildcard(&mask);

        assert_eq!(created_wildcard.get().as_vec(), vec![0, 0, 3, 255]);

        let mask: Address = Address::new(255, 255, 0, 0);
        let created_wildcard = wildcard(&mask);

        assert_eq!(created_wildcard.get().as_vec(), vec![0, 0, 255, 255]);

        let mask: Address = Address::new(255, 255, 128, 0);
        let created_wildcard = wildcard(&mask);

        assert_eq!(created_wildcard.get().as_vec(), vec![0, 0, 127, 255]);
    }

    #[test]
    fn test_mask_from_prefix() {
        let mut prefix: u8 = 24;
        let created_mask = mask_from_prefix(&mut prefix);
        let wanted_mask = Address::new(255, 255, 255, 0);
        assert_eq!(created_mask.get().as_vec(), wanted_mask.get().as_vec());

        let mut prefix: u8 = 30;
        let created_mask = mask_from_prefix(&mut prefix);
        let wanted_mask = Address::new(255, 255, 255, 252);
        assert_eq!(created_mask.get().as_vec(), wanted_mask.get().as_vec());

        let mut prefix: u8 = 20;
        let created_mask = mask_from_prefix(&mut prefix);
        let wanted_mask = Address::new(255, 255, 240, 0);
        assert_eq!(created_mask.get().as_vec(), wanted_mask.get().as_vec());

        let mut prefix: u8 = 0;
        let created_mask = mask_from_prefix(&mut prefix);
        let wanted_mask = Address::new(0, 0, 0, 0);
        assert_eq!(created_mask.get().as_vec(), wanted_mask.get().as_vec());

        let mut prefix: u8 = 32;
        let created_mask = mask_from_prefix(&mut prefix);
        let wanted_mask = Address::new(255, 255, 255, 255);
        assert_eq!(created_mask.get().as_vec(), wanted_mask.get().as_vec());
    }

    #[test]
    fn test_hosts_to_mask() {
        let hosts: u32 = 256;
        let mask = hosts_to_mask(hosts);
        assert_eq!(mask.as_vec(), vec![255, 255, 255, 0]);

        let hosts: u32 = 1024;
        let mask = hosts_to_mask(hosts);
        assert_eq!(mask.as_vec(), vec![255, 255, 252, 0]);

        let hosts: u32 = 4;
        let mask = hosts_to_mask(hosts);
        assert_eq!(mask.as_vec(), vec![255, 255, 255, 252]);
    }
    #[test]
    fn test_scaffold_hosts() {
        let base_net = Net::new(
            Address::new(192, 168, 0, 0),
            Address::new(255, 255, 255, 0));
        let hosts: Vec<u32> = vec![256, 4, 128, 32];

        let scaffolded: Vec<Net> = scaffold_hosts(&base_net, hosts);
        assert_eq!(scaffolded[0].get_network_address().as_vec(), [192, 168, 0, 0]);
        assert_eq!(scaffolded[1].get_network_address().as_vec(), [192, 168, 1, 0]);
        assert_eq!(scaffolded[2].get_network_address().as_vec(), [192, 168, 1, 128]);
        assert_eq!(scaffolded[3].get_network_address().as_vec(), [192, 168, 1, 160]);
        assert_eq!(scaffolded[3].get_broadcast().as_vec(), [192, 168, 1, 163]);

        let base_net = Net::new(
            Address::new(192, 168, 0, 0),
            Address::new(255, 255, 255, 0));
        let hosts: Vec<u32> = vec![256, 512, 1024];
        let scaffolded: Vec<Net> = scaffold_hosts(&base_net, hosts);
        assert_eq!(scaffolded[0].get_network_address().as_vec(), [192, 168, 0, 0]);
        assert_eq!(scaffolded[1].get_network_address().as_vec(), [192, 168, 4, 0]);
        assert_eq!(scaffolded[2].get_network_address().as_vec(), [192, 168, 6, 0]);
        assert_eq!(scaffolded[2].get_broadcast().as_vec(), [192, 168, 6, 255]);

        let base_net = Net::new(
            Address::new(192, 168, 0, 0),
            Address::new(255, 255, 255, 0));
        let hosts: Vec<u32> = vec![32, 64, 128];
        let scaffolded: Vec<Net> = scaffold_hosts(&base_net, hosts);
        assert_eq!(scaffolded[0].get_network_address().as_vec(), [192, 168, 0, 0]);
        assert_eq!(scaffolded[1].get_network_address().as_vec(), [192, 168, 0, 128]);
        assert_eq!(scaffolded[2].get_network_address().as_vec(), [192, 168, 0, 192]);
        assert_eq!(scaffolded[2].get_broadcast().as_vec(), [192, 168, 0, 223]);
    }
    #[test]
    fn test_scaffold_prefixes() {
        let base_net = Net::new(
            Address::new(192, 168, 0, 0),
            Address::new(255, 255, 255, 0));
        let hosts: Vec<u8> = vec![24, 30, 25, 27];

        let scaffolded: Vec<Net> = scaffold_prefixes(&base_net, hosts);
        assert_eq!(scaffolded[0].get_network_address().as_vec(), [192, 168, 0, 0]);
        assert_eq!(scaffolded[1].get_network_address().as_vec(), [192, 168, 1, 0]);
        assert_eq!(scaffolded[2].get_network_address().as_vec(), [192, 168, 1, 128]);
        assert_eq!(scaffolded[3].get_network_address().as_vec(), [192, 168, 1, 160]);
        assert_eq!(scaffolded[3].get_broadcast().as_vec(), [192, 168, 1, 163]);

        let base_net = Net::new(
            Address::new(192, 168, 0, 0),
            Address::new(255, 255, 255, 0));
        let hosts: Vec<u8> = vec![24, 23, 22];
        let scaffolded: Vec<Net> = scaffold_prefixes(&base_net, hosts);
        assert_eq!(scaffolded[0].get_network_address().as_vec(), [192, 168, 0, 0]);
        assert_eq!(scaffolded[1].get_network_address().as_vec(), [192, 168, 4, 0]);
        assert_eq!(scaffolded[2].get_network_address().as_vec(), [192, 168, 6, 0]);
        assert_eq!(scaffolded[2].get_broadcast().as_vec(), [192, 168, 6, 255]);

        let base_net = Net::new(
            Address::new(192, 168, 0, 0),
            Address::new(255, 255, 255, 0));
        let hosts: Vec<u8> = vec![27, 26, 25];
        let scaffolded: Vec<Net> = scaffold_prefixes(&base_net, hosts);
        assert_eq!(scaffolded[0].get_network_address().as_vec(), [192, 168, 0, 0]);
        assert_eq!(scaffolded[1].get_network_address().as_vec(), [192, 168, 0, 128]);
        assert_eq!(scaffolded[2].get_network_address().as_vec(), [192, 168, 0, 192]);
        assert_eq!(scaffolded[2].get_broadcast().as_vec(), [192, 168, 0, 223]);
    }
}
