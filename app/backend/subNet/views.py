from django.shortcuts import render
from .models import Net
from django.contrib.auth.models import User

from django.db import IntegrityError
from rest_framework import viewsets
from rest_framework.response import Response

from .serializers import NetSerializer

import rsnet
# Create your views here.


class NetViewSet(viewsets.ViewSet):
    def retrieve(self, request, pk=0):
        try:
            int(pk)
            parent = Net.objects.get(pk=pk)
        except:
            parent = Net.objects.get(title_slug=pk)
        children = Net.objects.filter(parent=parent)

        response = {
            "id": parent.id,
            "nets": {
                0: {
                    "name": parent.title,
                    "networkAddress": parent.network_address,
                    "broadcast": parent.broadcast,
                    "mask": parent.mask,
                    "wildcard": parent.wildcard,
                    "prefix": parent.prefix
                }
            }
        }

        iterator = 1
        for net in children:
            response["nets"][iterator] = {
                "networkAddress": net.network_address,
                "broadcast": net.broadcast,
                "mask": net.mask,
                "wildcard": net.wildcard,
                "prefix": net.prefix
            }
            iterator += 1

        return Response(response)


class PrefixPost(viewsets.ViewSet):
    def create(self, request, pk=0):
        base_address = request.data.get("baseAddress")
        for byte in base_address:
            try:
                int(byte)
            except:
                return Response({"ERROR": "Invalid number in base address"})
        mask = request.data.get("mask")
        for byte in mask:
            try:
                int(byte)
            except:
                return Response({"ERROR": "Invalid number in base address"})
        base_address = rsnet.Address(
            base_address[0], base_address[1], base_address[2], base_address[3])
        mask = rsnet.Address(
            mask[0], mask[1], mask[2], mask[3])
        base_net = rsnet.Net(base_address, mask)
        nets = rsnet.scaffold_prefixes(base_net, request.data.get("prefixes"))
        parent = Net(
            title=request.data.get("name"),
            network_address=".".join(
                str(netbyte) for netbyte in base_net.get_network_address().as_vec()),
            mask=".".join(
                str(netbyte) for netbyte in base_net.get_mask().as_vec()),
            broadcast=".".join(
                str(netbyte) for netbyte in base_net.get_broadcast().as_vec()),
            wildcard=".".join(
                str(netbyte) for netbyte in base_net.get_wildcard().as_vec()),
            prefix=base_net.get_prefix(),
        )
        try:
            parent.save()
        except IntegrityError:
            return Response({"error": "Invalid name"})
        response = {
            "id": parent.id,
            "name": parent.title,
        }
        iterator = 1
        for net in nets:
            child = Net(
                parent=parent,
                network_address=".".join(
                    str(netbyte) for netbyte in net.get_network_address().as_vec()),
                mask=".".join(
                    str(netbyte) for netbyte in net.get_mask().as_vec()),
                broadcast=".".join(
                    str(netbyte) for netbyte in net.get_broadcast().as_vec()),
                wildcard=".".join(
                    str(netbyte) for netbyte in net.get_wildcard().as_vec()),
                prefix=net.get_prefix(),
            )
            child.save()
            iterator += 1
        return Response(response)


class HostPost(viewsets.ViewSet):
    def create(self, request, pk=0):
        base_address = request.data.get("baseAddress")
        for byte in base_address:
            try:
                int(byte)
            except:
                return Response({"ERROR": "Invalid number in base address"})
        mask = request.data.get("mask")
        for byte in mask:
            try:
                int(byte)
            except:
                return Response({"ERROR": "Invalid number in base address"})
        base_address = rsnet.Address(
            base_address[0], base_address[1], base_address[2], base_address[3])
        mask = rsnet.Address(
            mask[0], mask[1], mask[2], mask[3])
        base_net = rsnet.Net(base_address, mask)
        nets = rsnet.scaffold_hosts(base_net, request.data.get("hosts"))
        parent = Net(
            title=request.data.get("name"),
            network_address=".".join(
                str(netbyte) for netbyte in base_net.get_network_address().as_vec()),
            mask=".".join(
                str(netbyte) for netbyte in base_net.get_mask().as_vec()),
            broadcast=".".join(
                str(netbyte) for netbyte in base_net.get_broadcast().as_vec()),
            wildcard=".".join(
                str(netbyte) for netbyte in base_net.get_wildcard().as_vec()),
            prefix=base_net.get_prefix(),
        )
        try:
            parent.save()
        except IntegrityError:
            return Response({"error": "Invalid name"})
        response = {
            "id": parent.id,
            "name": parent.title,
        }
        iterator = 1
        for net in nets:
            child = Net(
                parent=parent,
                network_address=".".join(
                    str(netbyte) for netbyte in net.get_network_address().as_vec()),
                mask=".".join(
                    str(netbyte) for netbyte in net.get_mask().as_vec()),
                broadcast=".".join(
                    str(netbyte) for netbyte in net.get_broadcast().as_vec()),
                wildcard=".".join(
                    str(netbyte) for netbyte in net.get_wildcard().as_vec()),
                prefix=net.get_prefix(),
            )
            child.save()
            iterator += 1
        return Response(response)
